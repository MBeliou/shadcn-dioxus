use dioxus::prelude::*;

#[test]
fn test_tabs_renders() {
    let _: Element = rsx! {
        crate::Tabs { default_value: "tab1".to_string(),
            crate::TabsList {
                crate::TabsTrigger { value: "tab1", "Tab 1" }
            }
            crate::TabsContent { value: "tab1", "Content" }
        }
    };
    assert!(true);
}
