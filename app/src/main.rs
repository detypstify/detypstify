pub mod model;
use std::f64;
use std::io::Cursor;

pub use model::mnist::Model;

use wasm_bindgen::prelude::*;

use burn::backend::Wgpu;
use dioxus::prelude::*;
use image::io::Reader as ImageReader;
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    type Backend = Wgpu;

    // let device = WgpuDevice::default();
    // let model = Model::default();
    // tracing::debug!("debug");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { class: "container mx-auto p-4",
            h1 { class: "text-4xl font-bold mb-4 text-center", "Detypstify" }
            div { class: "flex justify-center mb-4",
                canvas {
                    id: "canvas",
                    class: "border border-gray-700 bg-gray-800",
                    width: "400",
                    height: "300",
                    onmousedown: move |_| {
                        // start drawing
                        draw_smiley()
                    },
                    onmouseup: move |evt| {draw_smiley()},
                }
            }
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
                },
                "Clear"
            }
            div { class: "flex justify-center mb-4 button-like",
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
            div { id: "links",
                class: "flex justify-center space-x-4",
                a {
                    target: "_blank",
                    href: "https://github.com/yourusername",
                    class: "text-blue-400 hover:underline",
                    "GitHub"
                }
            }
        }
    }
}

pub fn draw_smiley() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
