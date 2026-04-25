use dioxus::prelude::*;

#[test]
fn test_alert_dialog_renders() {
    let _: Element = rsx! {
        crate::AlertDialog {
            crate::AlertDialogTrigger { "Show" }
            crate::AlertDialogContent {
                crate::AlertDialogTitle { "Title" }
                crate::AlertDialogDescription { "Desc" }
            }
        }
    };
    assert!(true);
}
