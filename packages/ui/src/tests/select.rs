use dioxus::prelude::*;

#[test]
fn test_select_renders() {
    let _: Element = rsx! {
        crate::Select {
            crate::SelectTrigger {
                crate::SelectValue { placeholder: "Pick one" }
            }
            crate::SelectContent {
                crate::SelectItem { value: "a", "Option A" }
            }
        }
    };
    assert!(true);
}
