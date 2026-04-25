use dioxus::prelude::*;

#[test]
fn test_card_renders() {
    let _: Element = rsx! {
        crate::Card {
            crate::CardHeader {
                crate::CardTitle { "Title" }
                crate::CardDescription { "Description" }
            }
            crate::CardContent { "Content" }
            crate::CardFooter { "Footer" }
        }
    };
    assert!(true);
}
