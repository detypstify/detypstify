pub mod model;
pub mod frontend;
pub mod inference;
use dioxus::launch;
use frontend::App;


use tracing::{debug, Level};
const HEIGHT: usize = 300;
const WIDTH: usize = 400;
const CHANNELS: usize = 3;

// TODO use globals for width and height of canvas

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    launch(App);
}


