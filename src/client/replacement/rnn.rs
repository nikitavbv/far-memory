use {
    std::collections::HashSet,
    tracing::info,
    candle_core::{Device, DType, Tensor, D},
    candle_nn::{rnn::{lstm, LSTMConfig, RNN, LSTM}, VarMap, VarBuilder, linear, Linear, Module, ops, loss, Optimizer},
};

pub struct RnnReplacementPolicy {
}

struct RNNModel {
    lstm_output_dim: usize,

    lstm: LSTM,
    linear: Linear,
}

impl RNNModel {
    pub fn new(vs: VarBuilder, classes: usize) -> Self {
        let lstm_output_dim = 10;
        let lstm = lstm(classes, lstm_output_dim, LSTMConfig::default(), vs.clone()).unwrap();
        let linear = linear(lstm_output_dim, classes, vs).unwrap();

        Self {
            lstm_output_dim,

            lstm,
            linear,
        }
    }

    pub fn forward(&self, data: &Tensor) -> Tensor {
        let lstm_output = self.lstm.seq(&data).unwrap();
        let lstm_output = self.lstm.states_to_tensor(&lstm_output).unwrap().reshape(&[lstm_output.len(), self.lstm_output_dim]).unwrap();
        ops::softmax_last_dim(&self.linear.forward(&lstm_output).unwrap()).unwrap()
    }
}

pub fn rnn_training_test() {
    info!("running rnn training");

    let dev = Device::cuda_if_available(0).unwrap();
    let varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &dev);

    let data: Vec<u32> = vec![1, 2, 3, 77, 45, 32, 1, 2, 3, 23, 44, 67, 89, 1, 2, 3, 123, 456, 28, 29, 1, 2, 3, 45, 32, 42, 1];
    let predictions = {
        let mut data = data.clone();
        data.remove(0);
        data.push(2);
        data
    };

    let data_len = data.len();
    let classes = data.iter().collect::<HashSet<_>>().len();
    let data: Vec<f32> = one_hot_encode(data).into_iter().flatten().collect();
    let data = Tensor::from_vec(data.clone(), &[1, data_len, classes], &dev).unwrap();

    let predictions_len = predictions.len();
    let predictions: Vec<f32> = one_hot_encode(predictions).into_iter().flatten().collect();
    let predictions = Tensor::from_vec(predictions.clone(), &[1, predictions_len, classes], &dev).unwrap();

    let model = RNNModel::new(vs.clone(), classes);
    let mut adam = candle_nn::AdamW::new_lr(varmap.all_vars(), 1.0).unwrap();
    for epoch in 0..100 {
        let output = model.forward(&data).reshape((1, 27, 15)).unwrap();
        let loss = loss::mse(&output, &predictions).unwrap();
        adam.backward_step(&loss).unwrap();

        println!("loss: {:?}", loss);
    }

    let result = model.forward(&data);
    println!("output: {:?}", result);
}

fn one_hot_encode(data: Vec<u32>) -> Vec<Vec<f32>> {
    let mut unqiue: Vec<_> = data.iter().collect::<HashSet<_>>().into_iter().collect();
    unqiue.sort();

    let mut result = Vec::new();
    for item in &data {
        let mut entry = vec![0.0; unqiue.len()];
        entry[unqiue.iter().position(|v| *v == item).unwrap()] = 1.0;
        result.push(entry);
    }

    result
}
