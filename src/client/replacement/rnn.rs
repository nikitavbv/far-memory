use {
    std::{collections::HashSet, fs, thread},
    tracing::info,
    candle_core::{Device, DType, Tensor},
    candle_nn::{rnn::{lstm, LSTMConfig, RNN, LSTM}, VarMap, VarBuilder, linear, Linear, Module, ops, loss, Optimizer},
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
    let predictions_len = predictions.len();
    let predictions = one_hot_encode(total_classes, predictions).into_iter().flatten().collect();
    let predictions = Tensor::from_vec(predictions, &[1, predictions_len, total_classes], &dev).unwrap();

    let data = {
        // remove last entry because we do not have prediction for it
        let mut data = data;
        data.remove(data.len() - 1);
        data
    };
    let data_len = data.len();
    let data = one_hot_encode(total_classes, data).into_iter().flatten().collect();
    let data = Tensor::from_vec(data, &[1, data_len, total_classes], &dev).unwrap();

    let model = RNNModel::new(vs.clone(), total_classes);
    let mut adam = candle_nn::AdamW::new_lr(varmap.all_vars(), 0.01).unwrap();

    // train
    for _epoch in 0..1000 {
        let output = model.forward(&data).reshape((1, data_len, total_classes)).unwrap();
        let loss = loss::mse(&output, &predictions).unwrap();
        adam.backward_step(&loss).unwrap();

        if loss.to_vec0::<f32>().unwrap() < 0.001 {
            break;
        }

        println!("loss: {:?}", loss);
    }

    // TODO: test

    /*let result = model.forward(&Tensor::from_vec(one_hot_encode(&keys, vec![77, 45, 1, 2, 3, 77, 45, 32, 1, 2]).into_iter().flatten().collect(), &[1, 10, classes], &dev).unwrap()).to_vec2::<f32>().unwrap();
    let result = &result[result.len() - 1];
    let result = result.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index)
        .unwrap();
    let result = keys[result];
    println!("output: {:?}", result);*/
}

fn one_hot_encode(total_classes: usize, data: Vec<u64>) -> Vec<Vec<f32>> {
    let mut result = Vec::new();
    for item in &data {
        let mut entry = vec![0.0; total_classes];
        entry[*item as usize] = 1.0;
        result.push(entry);
    }

    result
}
