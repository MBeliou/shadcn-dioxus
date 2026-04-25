use crate::cn;
use dioxus::prelude::*;

const LINK_BASE: &str = "h-9 w-9 inline-flex items-center justify-center rounded-md border border-input bg-background text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct PaginationLinkProps {
    #[props(into, default)]
    pub class: String,
    #[props(default)]
    pub href: Option<String>,
    #[props(default = false)]
    pub active: bool,
    pub children: Element,
}

#[component]
pub fn PaginationLink(props: PaginationLinkProps) -> Element {
    let classes = cn(LINK_BASE, &props.class);
    let active_class = if props.active { "bg-accent text-accent-foreground" } else { "" };
    let final_classes = cn(&classes, active_class);

    rsx! {
        li {
            a {
                href: props.href.as_deref().unwrap_or("#"),
                class: "{final_classes}",
                "aria-current": if props.active { "page" } else { "" },
                {props.children}
            }
        }
    }
}
