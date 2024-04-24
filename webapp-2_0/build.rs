use burn_import::onnx::{ModelGen, RecordType};

fn main() {
    if cfg!(feature = "embedded-model") {
        // If the embedded-model, then model is bundled into the binary.
        ModelGen::new()
            .input("/shared/typst-detexify/train/my_model_smol_1.onnx")
            .out_dir("model/")
            .record_type(RecordType::Bincode)
            .embed_states(true)
            .run_from_script();
    }

}

