pub mod frontend;
pub mod inference;
pub mod model;
pub mod typst;
pub mod typst_execute;
use dioxus::launch;
use frontend::App;

use tracing::Level;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    launch(App);
}
