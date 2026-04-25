use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbSeparatorProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn BreadcrumbSeparator(props: BreadcrumbSeparatorProps) -> Element {
    rsx! {
        span {
            class: "text-muted-foreground mx-1 {props.class}",
            "aria-hidden": "true",
            "/"
        }
    }
}
