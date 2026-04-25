use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PaginationContentProps {
    pub children: Element,
}

#[component]
pub fn PaginationContent(props: PaginationContentProps) -> Element {
    rsx! {
        ul { class: "flex flex-row items-center gap-1", {props.children} }
    }
}
