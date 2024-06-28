/*
This is my first time using Rust,
I am learning the language as I develop
this project. I am probably going to refactor
this code a bunch. Please be gentle ᴖ̈

       ＿＿
    🌸＞　　フ
    | 　_　 _l
    ／` ミ＿xノ
    /　　　 　 |
    /　 ヽ　　 ﾉ
    │　　|　|　|
／￣|　　 |　|　|
| (￣ヽ＿_ヽ_)__)
＼二つ


*/

struct LinearRegression {
    beta: f64,
    weight: Vec<f64>,
    learning_rate: f64,
    epochs: u64,
}

impl LinearRegression {
    // Predict on a trained model
    fn predict(&self, inputs: &[f64]) -> Option<f64> {
        if inputs.len() <= 0 {
            return None;
        }
        let mut result: f64 = 0.0;

        let mut index = 0;
        while index < inputs.len() {
            result = inputs[index] * self.weight[index] + result;
            index += 1;
        }

        result = result + self.beta;
        Some(result)
    }

    // Train the linear regression model
    fn train_model(&mut self, training_data: &Vec<Vec<f64>>, result: &Vec<f64>) -> () {
        if training_data.len() <= 0 {
            panic!("No valid training data to train on");
        }

        for _e in 0..self.epochs {
            let mut _mse: f64 = 0.0;

            for i in 0..training_data.len() - 1 {
                let _predicted_value = match self.predict(&training_data[i]) {
                    Some(prediction) => prediction,
                    _ => panic!("Couldn't predict any value"),
                };

                let error = _predicted_value - result[i];
                _mse = error * error + _mse;

                for j in 0..self.weight.len() - 1 {
                    self.weight[j] =
                        self.weight[j] - self.learning_rate * error * training_data[i][j];
                }

                self.beta = self.beta - self.learning_rate * error;
            }

            _mse = f64::sqrt(_mse) / training_data.len() as f64;
        }
    }

    // Function to build a new model
    fn model(beta: f64, feature_length: usize, learning_rate: f64, epochs: u64) -> Self {
        let weight: Vec<f64> = vec![0.0; feature_length];

        LinearRegression {
            beta,
            weight,
            learning_rate,
            epochs,
        }
    }
}

fn main() {
    let train_set = vec![
        vec![30.0],
        vec![16.0],
        vec![19.8],
        vec![18.4],
        vec![17.1],
        vec![15.5],
    ];
    let result = vec![88.6, 71.6, 93.3, 84.3, 80.6, 75.2];

    let mut _model = LinearRegression::model(0.0, train_set[0].len(), 0.001, 1000);

    _model.train_model(&train_set, &result);

    match _model.predict(&[30.0]) {
        Some(prediction) => println!("For 30.0, the model predicted: {prediction}"),
        None => println!("No prediction returned"),
    };
}
