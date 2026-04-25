use dioxus::prelude::*;

#[test]
fn test_breadcrumb_renders() {
    let _: Element = rsx! {
        crate::Breadcrumb {
            crate::BreadcrumbList {
                crate::BreadcrumbItem {
                    crate::BreadcrumbLink { href: "#", "Home" }
                }
                crate::BreadcrumbSeparator {}
                crate::BreadcrumbItem {
                    crate::BreadcrumbPage { "Current" }
                }
            }
        }
    };
    assert!(true);
}
