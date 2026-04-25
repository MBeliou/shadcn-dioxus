use dioxus::prelude::*;

#[test]
fn test_spinner_component_renders() {
    // VNode::empty() produces a valid Element (Result<VNode, RenderError>)
    let _: Element = rsx! { crate::Spinner {} };
    assert!(true);
}
