mod world;

use typst::{eval::Tracer, layout::Abs};
use world::TypstWrapperWorld;

pub fn set_svg(formula: &str) -> String {
    let content = format!("${}$", formula).to_owned();

    // Create world with content.
    let world = TypstWrapperWorld::new(content);

    // Render document
    let mut tracer = Tracer::default();
    let document = typst::compile(&world, &mut tracer).expect("Error compiling typst.");
    let svg = typst_svg::svg_merged(&document, Abs::pt(0.0));
    svg.replace("fill=\"#000000\"", "fill=\"#ffffff\"")
}
