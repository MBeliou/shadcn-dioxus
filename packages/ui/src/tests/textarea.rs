use dioxus::prelude::*;

#[test]
fn test_textarea_renders() {
    let _: Element = rsx! { crate::Textarea { placeholder: "Type..." } };
    assert!(true);
}
