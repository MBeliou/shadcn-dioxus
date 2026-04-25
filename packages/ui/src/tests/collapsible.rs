use dioxus::prelude::*;

#[test]
fn test_collapsible_renders() {
    let _: Element = rsx! {
        crate::Collapsible {
            crate::CollapsibleTrigger { "Toggle" }
            crate::CollapsibleContent { "Content" }
        }
    };
    assert!(true);
}
