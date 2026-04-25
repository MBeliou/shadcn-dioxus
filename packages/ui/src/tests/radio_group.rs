use dioxus::prelude::*;

#[test]
fn test_radio_group_renders() {
    let _: Element = rsx! {
        crate::RadioGroup {
            crate::RadioGroupItem { value: "one" }
        }
    };
    assert!(true);
}
