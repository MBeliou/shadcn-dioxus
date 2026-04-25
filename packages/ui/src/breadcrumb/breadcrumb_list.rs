use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbListProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn BreadcrumbList(props: BreadcrumbListProps) -> Element {
    rsx! {
        ol {
            class: "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5 {props.class}",
            {props.children}
        }
    }
}
