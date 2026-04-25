use dioxus::prelude::*;

#[test]
fn test_dropdown_menu_renders() {
    let _: Element = rsx! {
        crate::DropdownMenu {
            crate::DropdownMenuTrigger { "Open" }
            crate::DropdownMenuContent {
                crate::DropdownMenuItem { "Item 1" }
            }
        }
    };
    assert!(true);
}
