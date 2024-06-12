use js_sys::Uint8Array;
// use std::fs;
use tracing::debug;

// use typst::foundations::Smart;
use crate::typst::TypstWrapperWorld;
use typst::{eval::Tracer, layout::Abs};
use web_sys::{Blob, BlobPropertyBag, Url};

pub fn create_svg(_formula: &str) -> String {
    // TODO actually use formula
    let content = r#"
        $x^2 + 5x + 2 = 0$
    "#
    .to_owned();

    // Create world with content.
    let world = TypstWrapperWorld::new(content);

    // Render document
    let mut tracer = Tracer::default();
    let document = typst::compile(&world, &mut tracer).expect("Error compiling typst.");
    let svg_string = typst_svg::svg_merged(&document, Abs::pt(0.0));
    debug!("string svg: {}", svg_string);
    let svg_bytes = svg_string.as_bytes();
    let uint8_array = Uint8Array::from(svg_bytes);

    let mut options = BlobPropertyBag::new();
    options.type_("image/svg+xml");

    let blob = Blob::new_with_u8_array_sequence_and_options(
        &js_sys::Array::of1(&uint8_array.into()),
        &options,
    )
    .unwrap();

    Url::create_object_url_with_blob(&blob).unwrap()
}
