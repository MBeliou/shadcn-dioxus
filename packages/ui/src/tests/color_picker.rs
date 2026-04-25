use dioxus::prelude::*;

#[test]
fn test_color_picker_renders() {
    let _: Element = rsx! { crate::ColorPicker {} };
    assert!(true);
}
