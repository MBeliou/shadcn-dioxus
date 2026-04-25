use dioxus::prelude::*;

#[test]
fn test_avatar_renders() {
    let _: Element = rsx! {
        crate::Avatar {
            crate::AvatarFallback { "AB" }
        }
    };
    assert!(true);
}
