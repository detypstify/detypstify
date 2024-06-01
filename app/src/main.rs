pub mod model;
use std::f64;
use std::io::Cursor;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use burn::tensor::Tensor;
use burn_candle::Candle;
use dioxus::html::input_data::MouseButton;
pub use model::mnist::Model;

use wasm_bindgen::{prelude::*, Clamped};

use burn::backend::{NdArray};
use dioxus::prelude::*;
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
    //
#[derive(Debug, Clone)]
struct Point {
    x: Arc<AtomicU64>,
    y: Arc<AtomicU64>,
}

impl Point {
    fn new() -> Self {
        Self {
            x: Arc::default(),
            y: Arc::default(),
        }
    }

    fn get_coords(&self) -> (f64, f64) {
        (
            self.x.load(Ordering::SeqCst) as f64,
            self.y.load(Ordering::SeqCst) as f64,
        )
    }
}

fn set_position(event: MouseEvent, pos: Point) {
    let coords = event.element_coordinates();
    pos.x.store(coords.x as u64, Ordering::SeqCst);
    pos.y.store(coords.y as u64, Ordering::SeqCst);
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

fn draw(event: MouseEvent, pos: Point) -> Option<Clamped<Vec<u8>>> {
    if event.held_buttons().contains(MouseButton::Primary) {
        let coords = event.element_coordinates();
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap();
        let context = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context.begin_path();
        let (x, y) = pos.get_coords();
        context.move_to(x, y);
        set_position(event, pos);

        context.line_to(coords.x, coords.y);
        context.set_stroke_style(&JsValue::from_str("white"));
        context.set_line_width(5.0);
        context.stroke();

        Some(context.get_image_data(0.0, 0.0, 400.0, 300.0).unwrap().data())
    } else {
        None
    }
}

fn set_output(name: &str, res: &str) {
    let out = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(name)
        .unwrap();
    out.set_text_content(Some(res));
}

fn clear_outputs() {
    set_output("out1", "");
    set_output("out2", "");
    set_output("out3", "");
}

#[component]
fn OutputModel(name: String, res: String) -> Element {
    rsx! {
        div {
            class: "flex justify-left",
            h1 {
                class: "text-2xl font-bold ",
                "{res}"
            }
            p {id : "{name}",
            class: "ml-8 mt-1",
            ""
            }
        }
    }
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

#[component]
fn App() -> Element {
    let device = Default::default();
    let classifier = ImageClassifier { model: ModelType::WithNdArrayBackend(Model::new(&device))};
    let pos = Point::new();
    let pos_down = pos.clone();
    let pos_enter = pos.clone();
    let pos_move = pos.clone();

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"
        }
        section { class: "container max-w-fit",
            div { id: "header",
                class: "flex flex-row sticky items-center justify-center z-10",
                // img { src: "assets/logo.png", class: "w-24 h-24" }
                h1 {
                    class: "text-4xl font-bold mb-4",
                    style:"font-family:'0xProto Regular",
                    "Detypstify"
                }
            div {
                class: "flex justify-center",
                a {
                    target: "_blank",
                    href: "https://github.com/DieracDelta/detypstify",
                    i { class: "fa fa-github", }
                }
            }
            }
            div { class: "flex align-middle justify-center mb-4",
                canvas {
                    id: "canvas",
                    class: "border border-gray-700 bg-gray-800",
                    width: "400",
                    height: "300",
                    onmousedown:
                    move |evt| {
                      set_position(evt, pos_down.clone());
                    },
                    onmouseenter : move |evt| {
                      set_position(evt, pos_enter.clone());
                    },
                    onmouseup: move |_| {
                        // TODO: capture the image and run prediction
                        set_output("out1", "1. 0");
                    },
                    onmousemove: move |event| {
                        let maybe_canvas = draw(event, pos_move.clone());
                        if let Some(cv) = maybe_canvas {
                            use std::ops::Deref;
                            let cv2 : &[u8] = cv.deref();
                            inference(&classifier, cv2);
                        }

                    },
                }
            }
            div { class: "flex align-middle justify-center mb-4",
                div {
                    // make the class be dark and matching with the theme
                    class: "button-like",
                    tabindex: "0",
                    id: "btn",
                    onclick: move |_| {
                        let canvas = web_sys::window().unwrap().document().unwrap()
                            .get_element_by_id("canvas").unwrap();
                        let context = canvas.dyn_into::<web_sys::HtmlCanvasElement>()
                            .unwrap().get_context("2d").unwrap().unwrap()
                            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
                        context.clear_rect(0.0, 0.0, 400.0, 300.0);
                        clear_outputs();
                    },
                    style:"font-family:'0xProto Regular",
                    "Clear"
                }
                div { class: "flex justify-center mb-4 button-like",
                  style:"font-family:'0xProto Regular",
                  "Upload Image",
                    input {
                        r#type: "hidden",
                        id: "imageUpload",
                        accept: "image/*",
                        onchange: move |evt| {
                            async move {
                                    if let Some(file_engine) = evt.files() {
                                    let files = file_engine.files();
                                    for file_name in &files {
                                        if let Some(bytes) = file_engine.read_file(file_name).await
                                        {
                                            let image = ImageReader::new(Cursor::new(bytes))
                                                .with_guessed_format().unwrap()
                                                .decode().unwrap();
                                            tracing::debug!("Image width: {}, height: {}",
                                                image.width(), image.height());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            OutputModel { name: "out1", res: "1." }
            OutputModel { name: "out2", res: "2." }
            OutputModel { name: "out3", res: "3." }
        }
    }
}
