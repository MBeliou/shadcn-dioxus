use crate::cn;
use dioxus::prelude::*;
use lucide_dioxus::Ellipsis;

const BASE: &str = "h-9 w-9 inline-flex items-center justify-center";

#[derive(Clone, PartialEq, Props)]
pub struct PaginationEllipsisProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn PaginationEllipsis(props: PaginationEllipsisProps) -> Element {
    let classes = cn(BASE, &props.class);

    rsx! {
        li {
            span {
                class: "{classes}",
                "aria-hidden": "true",
                Ellipsis { class: "h-4 w-4" }
                span { class: "sr-only", "More pages" }
            }
        }
    }
}
