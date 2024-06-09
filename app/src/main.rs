pub mod model;
pub mod frontend;
use dioxus::launch;
use frontend::App;
use std::f64;
use std::io::Cursor;
use std::time::Instant;

use burn::tensor::Tensor;
use burn_candle::Candle;
pub use model::mnist::Model;


use burn::backend::{NdArray};
use image::io::Reader as ImageReader;
use tracing::{debug, Level};
const HEIGHT: usize = 300;
const WIDTH: usize = 400;
const CHANNELS: usize = 3;

// TODO use globals for width and height of canvas

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

use burn_wgpu::{AutoGraphicsApi, Wgpu};

pub enum ModelType {
    WithCandleBackend(Model<Candle<f32, i64>>),
    WithNdArrayBackend(Model<NdArray<f32>>),
    WithWgpuBackend(Model<Wgpu<AutoGraphicsApi, f32, i32>>),
}

pub struct ImageClassifier {
    model: ModelType
}


/// Returns the top 5 classes and convert them into a JsValue
fn top_5_classes(probabilities: Vec<f32>) -> Vec<InferenceResult> {
    // Convert the probabilities into a vector of (index, probability)
    let mut probabilities: Vec<_> = probabilities.iter().enumerate().collect();

    // Sort the probabilities in descending order
    probabilities.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    // Take the top 5 probabilities
    probabilities.truncate(5);

    // Convert the probabilities into InferenceResult
    probabilities
        .into_iter()
        .map(|(index, probability)| InferenceResult {
            index,
            probability: *probability,
            label: "todo".to_string()
        })
        .collect()
}

pub struct InferenceResult {
    index: usize,
    probability: f32,
    label: String,
}


fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    launch(App);
}

fn inference(model: &ImageClassifier, input: &[u8]) {
    // TODO fix
    // let mut ints = vec![];
    // for x in 0..HEIGHT {
    //     for y in 0..WIDTH {
    //         let idx = (y * width + x) * channels;
    //         let r = input[idx] as f32 / 255.0;
    //         let g = input[idx + 1] as f32 / 255.0;
    //         let b = input[idx + 2] as f32 / 255.0;
    //         let gray = 0.2989 * r + 0.5870 * g + 0.1140 * b;
    //         ints.push(gray);
    //     }
    // }
    //
    //
    // let start = Instant::now();
    // let result = match model.model {
    //     ModelType::WithCandleBackend(ref model) => {
    //         let input_tensor = Tensor::from_floats(ints, &Default::default()).reshape([1, CHANNELS, HEIGHT, WIDTH]);
    //         model.forward(input_tensor);
    //         todo!()
    //     },
    //     ModelType::WithNdArrayBackend(ref model) => {
    //         let input_tensor = Tensor::from_floats(ints, &Default::default()).reshape([1, CHANNELS, HEIGHT, WIDTH]);
    //         model.forward(input_tensor);
    //         todo!()
    //     },
    //     ModelType::WithWgpuBackend(ref model) => {
    //         let input_tensor = Tensor::from_floats(ints, &Default::default()).reshape([1, CHANNELS, HEIGHT, WIDTH]);
    //         model.forward(input_tensor);
    //         todo!()
    //     },
    // };
    // let duration = start.elapsed();
    // debug!("Inference is completed in {:?}", duration);
    //
    //
}

