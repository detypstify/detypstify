use wasm_bindgen::JsCast;
use web_sys::SvgsvgElement;

// use typst::foundations::Smart;
use crate::typst::TypstWrapperWorld;
use typst::{eval::Tracer, layout::Abs};
use web_sys::{DomParser, SupportedType};

// TODO rename
pub fn set_svg(formula: &str) -> String {
    let content = format!("${}$", formula).to_owned();

    // Create world with content.
    let world = TypstWrapperWorld::new(content);

    // Render document
    let mut tracer = Tracer::default();
    let document = typst::compile(&world, &mut tracer).expect("Error compiling typst.");
    typst_svg::svg_merged(&document, Abs::pt(0.0))
}

pub fn mutate_svg(formula: &str, svg_id: &str) {
    let resulting_svg = set_svg(formula);

    let parser = DomParser::new().unwrap();
    let svg_doc = parser
        .parse_from_string(&resulting_svg, SupportedType::ImageSvgXml)
        .unwrap();
    let svg_doc_ele = svg_doc.document_element().unwrap();
    tracing::error!("svg_doc: {:?}", svg_doc);
    tracing::error!("svg_doc_ele: {:?}", svg_doc_ele);
    let svg = svg_doc_ele.dyn_into::<SvgsvgElement>().unwrap();

    let body = web_sys::window().unwrap().document().unwrap();
    let container = body.get_element_by_id(svg_id).unwrap();
    while let Some(child) = container.first_child() {
        container
            .remove_child(&child)
            .expect("Failed to remove child");
    }
    container.append_child(&svg).unwrap();
    let bbox = svg.get_b_box().unwrap();
    svg.set_attribute(
        "viewBox",
        &format!(
            "{} {} {} {}",
            bbox.x(),
            bbox.y(),
            bbox.width(),
            bbox.height()
        ),
    )
    .unwrap();
    svg.set_attribute("width", "100").unwrap();
    svg.set_attribute("height", "100").unwrap();
}
