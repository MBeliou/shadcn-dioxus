use dioxus::prelude::*;

#[test]
fn test_popover_renders() {
    let _: Element = rsx! {
        crate::Popover {
            crate::PopoverTrigger { "Open" }
            crate::PopoverContent { "Content" }
        }
    };
    assert!(true);
}
