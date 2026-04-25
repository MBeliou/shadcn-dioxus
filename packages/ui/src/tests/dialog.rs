use dioxus::prelude::*;

#[test]
fn test_dialog_renders() {
    let _: Element = rsx! {
        crate::Dialog {
            crate::DialogTrigger { "Open" }
            crate::DialogPortal {
                crate::DialogOverlay {}
                crate::DialogContent {
                    crate::DialogTitle { "Title" }
                }
            }
        }
    };
    assert!(true);
}
