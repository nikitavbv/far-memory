use {
    std::{collections::HashSet, fs, thread, time::Instant},
    tracing::{info, span, Level},
    candle_core::{Device, DType, Tensor},
    candle_nn::{rnn::{lstm, LSTMConfig, RNN, LSTM}, VarMap, VarBuilder, linear, Linear, Module, ops, loss, Optimizer},
    rand::seq::SliceRandom,
    crate::manager::{SpanAccessEvent, ManagerClient, ReplacementPolicyType},
    super::{ReplacementPolicy, SpanId, TrackingReplacementPolicy, RandomReplacementPolicy},
};

pub struct RnnReplacementPolicy {
    fallback: Box<dyn ReplacementPolicy>,
    tracking: TrackingReplacementPolicy,

    model: Option<RNNModel>,
}

impl RnnReplacementPolicy {
    pub fn new(manager: ManagerClient, fallback: Box<dyn ReplacementPolicy>) -> Self {
        let model = manager.get_replacement_policy_params(ReplacementPolicyType::RNN).rnn_weights;
        let model = model.map(|weights| RNNModel::from_weights(weights.total_spans, Device::cuda_if_available(0).unwrap(), weights.weights));

        Self {
            fallback,

            // tracking is needed to collect fresh stats for next rnn training runs.
            tracking: TrackingReplacementPolicy::new(manager, Box::new(RandomReplacementPolicy::new())),

            model,
        }
    }

    pub fn train_rnn_model(data: Vec<SpanAccessEvent>) -> Vec<u8> {
        train_rnn_model(data).export_weights()
    }
}

impl ReplacementPolicy for RnnReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        if let Some(model) = &self.model {
            // if all spans are within bounds of the model.
            if spans.iter().map(|span| span.id()).find(|id| *id >= model.total_spans).is_none() {
                // TODO: pick using RNN
                unimplemented!()
            }
        }

        self.fallback.pick_for_eviction(spans)
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.tracking.on_span_access(span_id);
    }

    fn on_span_swap_out(&self, span_id: &SpanId) {
        self.tracking.on_span_swap_out(span_id);
    }

    fn on_span_swap_in(&self, span_id: &SpanId) {
        self.tracking.on_span_swap_in(span_id);
    }

    fn on_stop(&self) {
        self.tracking.on_stop();
    }
}

struct RNNModel {
    lstm_output_dim: usize,
    total_spans: u64,

    varmap: VarMap,
    lstm: LSTM,
    linear: Linear,
}

impl RNNModel {
    pub fn new(varmap: VarMap, vs: VarBuilder, total_spans: u64) -> Self {
        // span = class, so total_spans is number of classes.
        let lstm_output_dim = 10;
        let lstm = lstm(total_spans as usize, lstm_output_dim, LSTMConfig::default(), vs.clone()).unwrap();
        let linear = linear(lstm_output_dim, total_spans as usize, vs).unwrap();

         Self {
            lstm_output_dim,
            total_spans,

            varmap,
            lstm,
            linear,
        }
    }

    pub fn from_weights(total_spans: u64, dev: Device, weights: Vec<u8>) -> Self {
        let mut varmap = VarMap::new();

        let path = "./data/rnn_weights.safetensors";
        std::fs::write(path, weights).unwrap();
        varmap.load(path).unwrap();

        let vs = VarBuilder::from_varmap(&varmap, DType::F32, &dev);

        Self::new(varmap, vs, total_spans)
    }

    pub fn export_weights(&self) -> Vec<u8> {
        let path = "./data/rnn_weights.safetensors";
        self.varmap.save(path).unwrap();
        std::fs::read(path).unwrap()
    }

    pub fn forward(&self, data: &Tensor) -> Tensor {
        let lstm_output = self.lstm.seq(&data).unwrap();
        let lstm_output = self.lstm.states_to_tensor(&lstm_output).unwrap().reshape(&[lstm_output.len(), self.lstm_output_dim]).unwrap();
        self.linear.forward(&lstm_output).unwrap()
    }
}

fn train_rnn_model(data: Vec<SpanAccessEvent>) -> RNNModel {
    info!("running rnn training");

    // of course, this implementation is not optimal. The goal here is to demonstrate the idea.
    let dev = Device::cuda_if_available(0).unwrap();
    let varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &dev);

    let data: Vec<u64> = data.iter().map(|v| v.span_id).collect();

    let total_classes = *data.iter().max().unwrap() + 1; // +1 because start at zero
    let predictions = {
        let mut data = data.clone();
        data.remove(0);
        data
    };

    let data = {
        // remove last entry because we do not have prediction for it
        let mut data = data;
        data.remove(data.len() - 1);
        data
    };

    let model = RNNModel::new(varmap.clone(), vs.clone(), total_classes);
    let mut adam = candle_nn::AdamW::new_lr(varmap.all_vars(), 0.1).unwrap();

    // train
    let window_size = 100;
    for epoch in 0..1000 {
        let started_at = Instant::now();
        span!(Level::DEBUG, "training epoch", epoch = epoch).in_scope(|| {
            let mut starting_points: Vec<usize> = (0..data.len()-window_size-1).collect();
            starting_points.shuffle(&mut rand::thread_rng());

            let mut total_loss = 0.0;
            let mut batch = 0;

            for point in &starting_points {
                span!(Level::DEBUG, "batch", point, batch, total_batches = starting_points.len()).in_scope(|| {
                    let (input_data, output_data) = span!(Level::DEBUG, "prepare data").in_scope(|| {
                        let input_data = one_hot_encode(total_classes, &data[*point..point+window_size]).into_iter().flatten().collect();
                        let input_data = Tensor::from_vec(input_data, &[1, window_size, total_classes as usize], &dev).unwrap();

                        let output_data = one_hot_encode(total_classes, &predictions[point + 1..point + 1 + window_size]).into_iter().flatten().collect();
                        let output_data = Tensor::from_vec(output_data, &[1, window_size, total_classes as usize], &dev).unwrap();

                        (input_data, output_data)
                    });

                    let output = span!(Level::DEBUG, "model forward").in_scope(|| model.forward(&input_data).reshape((1, window_size, total_classes as usize)).unwrap());
                    let loss = loss::mse(&output, &output_data).unwrap();
                    span!(Level::DEBUG, "optimizer step").in_scope(|| adam.backward_step(&loss).unwrap());

                    total_loss += loss.to_vec0::<f32>().unwrap();

                    span!(Level::DEBUG, "drop data").in_scope(move || {
                       std::mem::drop(input_data);
                       std::mem::drop(output_data);
                       std::mem::drop(output);
                       std::mem::drop(loss);
                    });
                });
                batch += 1;
            }

            // not good to do this without test set, but good for now anyway.
            let mut test_starting_points: Vec<usize> = (0..data.len()-window_size-1).collect();
            test_starting_points.shuffle(&mut rand::thread_rng());

            let mut correct_predictions = 0;
            for point in &test_starting_points {
                let input_data = one_hot_encode(total_classes, &data[*point..point+window_size]).into_iter().flatten().collect();
                let input_data = Tensor::from_vec(input_data, &[1, window_size, total_classes as usize], &dev).unwrap();

                let output_data = model.forward(&input_data).to_vec2::<f32>().unwrap();
                let output_data = &output_data[output_data.len() - 1];
                let result = output_data.iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                    .map(|(index, _)| index)
                    .unwrap();

                if result as u64 == data[point + 1] {
                    correct_predictions += 1;
                }
            }

            println!("epoch: {}, loss: {}, test accuracy: {}%, time per epoch: {}", epoch, total_loss, (correct_predictions as f32 / test_starting_points.len() as f32 * 100.0) as u32, (Instant::now() - started_at).as_secs());
        });
    }

    // test
    let mut correct_predictions = 0;

    for i in 1..data.len()-1 {
        let input = one_hot_encode(total_classes, &data[0..i]).into_iter().flatten().collect();
        let input = Tensor::from_vec(input, &[1, i, total_classes as usize], &dev).unwrap();
        let result = model.forward(&input).to_vec2::<f32>().unwrap();
        let result = &result[result.len() - 1];
        let result = result.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        if result as u64 == data[i + 1] {
            correct_predictions += 1;
        }

        println!("{} {} {}", result, data[i + 1], correct_predictions);
    }

    model
}

pub fn rnn_training_test() {
    let data: Vec<SpanAccessEvent> = serde_json::from_slice(&fs::read("./data/span_access_stats.json").unwrap()).unwrap();
    train_rnn_model(data);
}

fn one_hot_encode(total_classes: u64, data: &[u64]) -> Vec<Vec<f32>> {
    let mut result = Vec::new();
    for item in data {
        let mut entry = vec![0.0; total_classes as usize];
        entry[*item as usize] = 1.0;
        result.push(entry);
    }

    result
}
