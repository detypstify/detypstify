use crate::model::mnist::Model;
use burn_wgpu::{AutoGraphicsApi, Wgpu};
use js_sys::Array;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};


use burn::backend::NdArray;
use burn::tensor::Tensor;
use burn_candle::Candle;

#[derive(Clone)]
pub enum MLBackend {
    Candle(Model<Candle<f32, i64>>),
    NdArray(Model<NdArray<f32>>),
    Wgpu(Model<Wgpu<AutoGraphicsApi, f32, i32>>),
}

#[derive(Clone)]
pub struct ImageClassifier {
    pub model: MLBackend,
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
            label: "todo".to_string(),
        })
        .collect()
}

pub struct InferenceResult {
    index: usize,
    probability: f32,
    label: String,
}

pub async fn inference(model: &ImageClassifier, input: &[f32]) -> Vec<u32> {
    match model.model {
        MLBackend::Candle(ref model) => {
            type Backend = Candle<f32, i64>;
            let device = Default::default();
            // NOTE might be able to lift the resizing up in scope slightly
            let input: Tensor<Backend, 3> = Tensor::from_floats(input, &device).reshape([1, 28, 28]);
            let input = ((input / 255) - 0.1307) / 0.3081;
            // let output : Tensor<Backend, 2> = model.forward(input);
            todo!()
        },
        MLBackend::NdArray(ref model) => {
            type Backend = NdArray<f32>;
            let device = Default::default();
            // NOTE might be able to lift the resizing up in scope slightly
            let input: Tensor<Backend, 4> = Tensor::from_floats(input, &device).reshape([1, 1, 28, 28]);
            let input = ((input / 255) - 0.1307) / 0.3081;
            let output : Tensor<Backend, 2> = model.forward(input);
            let sorted_results = output.argsort(1).await;
            let output = sorted_results.into_data().await.convert::<u32>().value;
            output.into_iter().take(3).collect()
        },
        MLBackend::Wgpu(ref model) => {
            type Backend = Wgpu<AutoGraphicsApi, f32, i32>;
            todo!()
        },
    }
}

pub fn process_data(ctx: &CanvasRenderingContext2d) -> Option<Vec<f32>> {
    let canvas = ctx.canvas().unwrap();
    let width = canvas.width();
    let height = canvas.height();

    let image_data = ctx.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();

    if let Some((crop_x, crop_y, crop_width, crop_height)) = find_bounds(&image_data) {
        tracing::error!("crop_x: {}, crop_y: {}, crop_width: {}, crop_height: {}", crop_x, crop_y, crop_width, crop_height);
        let cropped_data = ctx.get_image_data(crop_x as f64, crop_y as f64, crop_width as f64, crop_height as f64).unwrap();

        let scaled_data = scale_image_data_to_28x28(&cropped_data).unwrap();

        Some(rgba_to_gray(&scaled_data))
    } else {
        None
    }
}

fn find_bounds(image_data: &ImageData) -> Option<(usize, usize, usize, usize)> {
    let data = image_data.data();
    let width = image_data.width() as usize;
    let height = image_data.height() as usize;
    let mut min_x = width;
    let mut max_x = 0;
    let mut min_y = height;
    let mut max_y = 0;

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            if data[idx + 3] != 0 { // Check alpha channel to find non-transparent pixels
                if x < min_x { min_x = x; }
                if x > max_x { max_x = x; }
                if y < min_y { min_y = y; }
                if y > max_y { max_y = y; }
            }
        }
    }
    // Check if any non-transparent pixel was found
    if min_x <= max_x && min_y <= max_y {
        Some((min_x, min_y, max_x - min_x + 1, max_y - min_y + 1))
    } else {
        // No non-transparent pixels found, return None
        None
    }
}

fn scale_image_data_to_28x28(image_data: &ImageData) -> Result<ImageData, JsValue> {
    let src_width = image_data.width() as usize;
    let src_height = image_data.height() as usize;
    let target_width = 28;
    let target_height = 28;

    // Create a new buffer for the scaled image
    let mut scaled_data = vec![0u8; target_width * target_height * 4]; // RGBA data

    for y in 0..target_height {
        for x in 0..target_width {
            let src_x = x * src_width / target_width;
            let src_y = y * src_height / target_height;
            let src_idx = (src_y * src_width + src_x) * 4;

            let dst_idx = (y * target_width + x) * 4;
            for i in 0..4 {
                scaled_data[dst_idx + i] = image_data.data()[src_idx + i];
            }
        }
    }

    // Create Clamped array from scaled_data
    let data_clamped: Clamped<&[u8]> = Clamped(&scaled_data);

    // Create and return new ImageData from scaled_data
    ImageData::new_with_u8_clamped_array_and_sh(data_clamped, target_width as u32, target_height as u32)
}


fn rgba_to_gray(image_data: &ImageData) -> Vec<f32> {
    let data = image_data.data();
    let mut gray_data = Vec::new();

    for i in (0..data.len()).step_by(4) {
        let r = (*data)[i] as f32;
        let g = (*data)[i+1] as f32;
        let b = (*data)[i+2] as f32;
        // Convert to grayscale using luminosity method
        let gray = 0.299 * r + 0.587 * g + 0.114 * b;
        gray_data.push(255.0 - gray); // Invert grayscale to match the JavaScript behavior
    }

    gray_data
}

