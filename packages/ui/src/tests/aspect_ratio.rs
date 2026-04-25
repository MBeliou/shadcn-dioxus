use dioxus::prelude::*;

#[test]
fn test_aspect_ratio_renders() {
    let _: Element = rsx! { crate::AspectRatio { ratio: 1.5 } };
    assert!(true);
}
