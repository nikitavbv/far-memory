use {
    std::{collections::HashSet, fs, thread},
    tracing::{info, span, Level},
    candle_core::{Device, DType, Tensor},
    candle_nn::{rnn::{lstm, LSTMConfig, RNN, LSTM}, VarMap, VarBuilder, linear, Linear, Module, ops, loss, Optimizer},
    rand::seq::SliceRandom,
    crate::manager::SpanAccessEvent,
};

pub struct RnnReplacementPolicy {
}

struct RNNModel {
    lstm_output_dim: usize,

    lstm: LSTM,
    linear: Linear,
}

impl RNNModel {
    pub fn new(vs: VarBuilder, total_spans: usize) -> Self {
        // span = class, so total_spans is number of classes.
        let lstm_output_dim = 10;
        let lstm = lstm(total_spans, lstm_output_dim, LSTMConfig::default(), vs.clone()).unwrap();
        let linear = linear(lstm_output_dim, total_spans, vs).unwrap();

        Self {
            lstm_output_dim,

            lstm,
            linear,
        }
    }

    pub fn forward(&self, data: &Tensor) -> Tensor {
        let lstm_output = self.lstm.seq(&data).unwrap();
        let lstm_output = self.lstm.states_to_tensor(&lstm_output).unwrap().reshape(&[lstm_output.len(), self.lstm_output_dim]).unwrap();
        self.linear.forward(&lstm_output).unwrap()
    }
}

pub fn rnn_training_test() {
    info!("running rnn training");

    thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(rnn_training).unwrap().join().unwrap();
}

fn rnn_training() {
    // of course, this implementation is not optimal. The goal here is to demonstrate the idea.
    let dev = Device::cuda_if_available(0).unwrap();
    let varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &dev);

    let data: Vec<SpanAccessEvent> = serde_json::from_slice(&fs::read("./data/span_access_stats.json").unwrap()).unwrap();
    let data: Vec<u64> = data.iter().map(|v| v.span_id).collect();

    let total_classes = *data.iter().max().unwrap() as usize + 1; // +1 because start at zero
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

    let model = RNNModel::new(vs.clone(), total_classes);
    let mut adam = candle_nn::AdamW::new_lr(varmap.all_vars(), 0.01).unwrap();

    // train
    let window_size = 100;
    for epoch in 0..1000 {
        span!(Level::DEBUG, "training epoch", epoch = epoch).in_scope(|| {
            let mut starting_points: Vec<usize> = (0..data.len()-window_size-1).collect();
            starting_points.shuffle(&mut rand::thread_rng());

            let mut total_loss = 0.0;
            let mut batch = 0;

            for point in &starting_points {
                span!(Level::DEBUG, "batch", point, batch, total_batches = starting_points.len()).in_scope(|| {
                    let (input_data, output_data) = span!(Level::DEBUG, "prepare data").in_scope(|| {
                        let input_data = one_hot_encode(total_classes, &data[*point..point+window_size]).into_iter().flatten().collect();
                        let input_data = Tensor::from_vec(input_data, &[1, window_size, total_classes], &dev).unwrap();

                        let output_data = one_hot_encode(total_classes, &predictions[point + 1..point + 1 + window_size]).into_iter().flatten().collect();
                        let output_data = Tensor::from_vec(output_data, &[1, window_size, total_classes], &dev).unwrap();

                        (input_data, output_data)
                    });

                    let output = span!(Level::DEBUG, "model forward").in_scope(|| model.forward(&input_data).reshape((1, window_size, total_classes)).unwrap());
                    let loss = loss::mse(&output, &output_data).unwrap();
                    adam.backward_step(&loss).unwrap();

                    total_loss += loss.to_vec0::<f32>().unwrap();
                });
                batch += 1;
            }

            println!("epoch: {}, loss: {}", epoch, total_loss);
        });
    }

    // test
    let mut correct_predictions = 0;

    for i in 1..data.len()-1 {
        let input = one_hot_encode(total_classes, &data[0..i]).into_iter().flatten().collect();
        let input = Tensor::from_vec(input, &[1, i, total_classes], &dev).unwrap();
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
}

fn one_hot_encode(total_classes: usize, data: &[u64]) -> Vec<Vec<f32>> {
    let mut result = Vec::new();
    for item in data {
        let mut entry = vec![0.0; total_classes];
        entry[*item as usize] = 1.0;
        result.push(entry);
    }

    result
}
