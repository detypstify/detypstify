use burn_import::onnx::{ModelGen, RecordType};

const INPUT_ONNX_FILE: &str = "src/model/mnist.onnx";
const OUT_DIR: &str = "model/";

fn main() {
    ModelGen::new()
        .input(INPUT_ONNX_FILE)
        .out_dir(OUT_DIR)
        .record_type(RecordType::Bincode)
        .embed_states(true)
        .run_from_script();
}
