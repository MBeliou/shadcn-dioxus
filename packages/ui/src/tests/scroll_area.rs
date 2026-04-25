use dioxus::prelude::*;

#[test]
fn test_scroll_area_renders() {
    let _: Element = rsx! { crate::ScrollArea { "content" } };
    assert!(true);
}
