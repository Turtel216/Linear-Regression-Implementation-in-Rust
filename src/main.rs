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
    beta: f32,
    weight: Vec<f32>,
    learning_rate: f32,
    epochs: i64,
}

impl LinearRegression {
    // Predict on a trained model
    fn predict(&self, inputs: &[f32]) -> Option<f32> {
        if inputs.len() <= 0 {
            return None;
        }
        let mut result: f32 = 0.0;

        let mut index = 0;
        while index < inputs.len() {
            result = inputs[index] * self.weight[index] + result;
            index += 1;
        }

        result = result + self.beta;
        Some(result)
    }

    // Train the linear regression model
    fn train_model(&mut self, training_data: Vec<Vec<f32>>, result: Vec<f32>) -> () {
        if training_data.len() <= 0 {
            // throw error
        }

        for _e in 0..self.epochs {
            let mut _mse: f32 = 0.0;

            for i in 0..training_data.len() {
                let temp_input: &[f32] = &training_data[i];

                let _predicted_value = match self.predict(temp_input) {
                    Some(prediction) => prediction,
                    _ => 0.0, //TODO handle error
                };

                let error = _predicted_value - result[i];
                _mse = error * error + _mse;

                for j in 0..self.weight.len() {
                    self.weight[j] = self.weight[j] - self.learning_rate * error * temp_input[j];
                }

                self.beta = self.beta - self.learning_rate * error;
            }

            _mse = f32::sqrt(_mse) / training_data.len() as f32;
        }
    }

    // Function to build a new model
    fn model(beta: f32, weight: Vec<f32>, learning_rate: f32, epochs: i64) -> Self {
        LinearRegression {
            beta,
            weight,
            learning_rate,
            epochs,
        }
    }
}

fn main() {}
