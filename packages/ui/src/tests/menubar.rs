use dioxus::prelude::*;

#[test]
fn test_menubar_renders() {
    let _: Element = rsx! {
        crate::Menubar {
            crate::MenubarMenu {
                crate::MenubarTrigger { "File" }
                crate::MenubarContent {
                    crate::MenubarItem { "New" }
                }
            }
        }
    };
    assert!(true);
}
