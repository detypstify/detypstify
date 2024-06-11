pub mod frontend;
pub mod inference;
pub mod model;
use dioxus::launch;
use frontend::App;

use tracing::Level;
// TODO use globals for width and height of canvas

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    launch(App);
}
