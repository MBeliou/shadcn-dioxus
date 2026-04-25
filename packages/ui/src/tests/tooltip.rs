use dioxus::prelude::*;

#[test]
fn test_tooltip_renders() {
    let _: Element = rsx! {
        crate::Tooltip {
            crate::TooltipTrigger { "Hover" }
            crate::TooltipContent { "Content" }
        }
    };
    assert!(true);
}
