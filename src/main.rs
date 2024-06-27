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
    fn train_model(&mut self, training_data: &Vec<Vec<f32>>, result: &Vec<f32>) -> () {
        if training_data.len() <= 0 {
            // throw error
        }

        for _e in 0..self.epochs {
            let mut _mse: f32 = 0.0;

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

            _mse = f32::sqrt(_mse) / training_data.len() as f32;
        }
    }

    // Function to build a new model
    fn model(beta: f32, feature_length: usize, learning_rate: f32, epochs: i64) -> Self {
        let mut weight: Vec<f32> = vec![];

        for _i in 0..feature_length {
            weight.push(0.0);
        }
        LinearRegression {
            beta,
            weight,
            learning_rate,
            epochs,
        }
    }
}

fn main() {
    let train_set = [
        [30.0].to_vec(),
        [16.0].to_vec(),
        [19.8].to_vec(),
        [18.4].to_vec(),
        [17.1].to_vec(),
        [15.5].to_vec(),
    ]
    .to_vec();
    let result = [88.6, 71.6, 93.3, 84.3, 80.6, 75.2].to_vec();

    let mut _model = LinearRegression::model(0.0, 1, 0.001, 1000);

    _model.train_model(&train_set, &result);

    match _model.predict(&[20.0]) {
        Some(prediction) => println!("For 20.0, the model predicted: {prediction}"),
        None => println!("No prediction returned"),
    };
}
