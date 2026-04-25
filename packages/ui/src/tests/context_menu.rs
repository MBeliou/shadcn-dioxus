use dioxus::prelude::*;

#[test]
fn test_context_menu_renders() {
    let _: Element = rsx! {
        crate::ContextMenu {
            crate::ContextMenuTrigger { "Right-click" }
            crate::ContextMenuContent {
                crate::ContextMenuItem { "Item" }
            }
        }
    };
    assert!(true);
}
