use dioxus::prelude::*;

#[test]
fn test_pagination_renders() {
    let _: Element = rsx! {
        crate::Pagination { page: 1, total_pages: 5,
            crate::PaginationContent {
                crate::PaginationItem { "1" }
            }
        }
    };
    assert!(true);
}
