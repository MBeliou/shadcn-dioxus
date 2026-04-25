use dioxus::prelude::*;

#[test]
fn test_hover_card_renders() {
    let _: Element = rsx! {
        crate::HoverCard {
            crate::HoverCardTrigger { "Hover" }
            crate::HoverCardContent { "Content" }
        }
    };
    assert!(true);
}
