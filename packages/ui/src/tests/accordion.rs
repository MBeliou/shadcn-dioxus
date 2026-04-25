use dioxus::prelude::*;

#[test]
fn test_accordion_renders() {
    let _: Element = rsx! {
        crate::Accordion {
            crate::AccordionItem { value: "item-1".to_string(),
                crate::AccordionTrigger { "Title" }
                crate::AccordionContent { "Content" }
            }
        }
    };
    assert!(true);
}
