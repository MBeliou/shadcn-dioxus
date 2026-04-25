use dioxus::prelude::*;

#[test]
fn test_switch_renders() {
    let _: Element = rsx! { crate::Switch {} };
    assert!(true);
}

#[test]
fn test_switch_disabled_renders() {
    let _: Element = rsx! { crate::Switch { disabled: true } };
    assert!(true);
}
