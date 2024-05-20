pub mod model;
use burn::backend::{wgpu::WgpuDevice, Wgpu};
pub use model::mnist::Model;

use dioxus::prelude::*;
use tracing::{event, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    type Backend = Wgpu;

    let device = WgpuDevice::default();
    let model = Model::default();
    tracing::debug!("debug");
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut input_file: Signal<Vec<String>> = use_signal(Vec::new);

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        // img { src: "header.svg", id: "header" }
        div {
            id: "links",
            p { "Upload your file" }
        }
        input {
            // tell the input to pick a file
            r#type: "file",
            // list the accepted extensions
            accept: ".png",
            // pick multiple files
            multiple: false,
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
}
