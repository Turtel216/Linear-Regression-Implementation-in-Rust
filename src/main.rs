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
    weight: [f32],
    learning_rate: f32,
    epochs: i64,
}

impl LinearRegression {
    // Prediect on a trained model
    fn predict(&self, inputs: &[f32]) -> f32 {
        //TODO Throw error when input length is <= 0
        let mut result: f32 = 0.0;

        let mut index = 0;
        while index < inputs.len() {
            result = inputs[index] * self.weight[index] + result;
            index += 1;
        }

        result = result + self.beta;
        result
    }

    // Train the linear regression model
    fn trainModel(&self, train_data: [f32], result: [32]) -> () {}

    // Function to build a new model
    fn model(beta: f32, weight: [f32], learning_rate: f32, epochs: i64) -> Self {
        LinearRegression {
            beta,
            weight,
            learning_rate,
            epochs,
        }
}

}

fn main() {}
