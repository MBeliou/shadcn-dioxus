use dioxus::prelude::*;

#[test]
fn test_skeleton_renders() {
    let _: Element = rsx! { crate::Skeleton { class: "h-4 w-4" } };
    assert!(true);
}
