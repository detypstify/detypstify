pub mod model;
use burn::backend::Wgpu;
pub use model::mnist::Model;

use dioxus::prelude::*;
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
    // Build cool things ✌️
    let mut input_file: Signal<Vec<String>> = use_signal(Vec::new);

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { class: "container mx-auto p-4",
            h1 { class: "text-4xl font-bold mb-4 text-center", "Detypstify" }
            div { class: "flex justify-center mb-4",
                canvas {
                    id: "drawCanvas",
                    class: "border border-gray-700 bg-gray-800",
                    width: "800",
                    height: "600",
                }
            }
            label {
                class: "cursor-pointer bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                "Upload Image"
            }
            div { class: "flex justify-center mb-4",
                input {
                    r#type: "file",
                    id: "imageUpload",
                    class: "hidden",

                    accept: "image/*",
                    onchange: move |evt| {
                        async move {
                                if let Some(file_engine) = evt.files() {
                                let files = file_engine.files();
                                for file_name in &files {
                                    if let Some(file) = file_engine.read_file_to_string(file_name).await
                                    {
                                        input_file.write().push(file);
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
                a {
                    target: "_blank",
                    href: "https://otherlink.com",
                    class: "text-blue-400 hover:underline",
                    "Other Link"
                }
            }
        }
    }
}
