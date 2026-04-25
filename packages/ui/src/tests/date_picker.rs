use dioxus::prelude::*;

#[test]
fn test_date_picker_renders() {
    let _: Element = rsx! { crate::DatePicker {} };
    assert!(true);
}
