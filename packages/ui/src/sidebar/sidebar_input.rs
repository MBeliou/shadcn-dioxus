use crate::cn;
use dioxus::prelude::*;
use lucide_dioxus::Search;

const INPUT_BASE: &str = "h-8 w-full rounded-md border border-sidebar-border bg-transparent px-3 text-sm outline-none ring-sidebar-ring placeholder:text-sidebar-foreground/50 focus-visible:ring-2";

#[derive(Clone, PartialEq, Props)]
pub struct SidebarInputProps {
    #[props(default = "Search...".to_string())]
    pub placeholder: String,
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarInput(props: SidebarInputProps) -> Element {
    rsx! {
        div {
            class: "relative",
            Search { class: "absolute left-2 top-1/2 h-4 w-4 -translate-y-1/2 text-sidebar-foreground/50" }
            input {
                r#type: "text",
                "data-slot": "sidebar-input",
                class: cn(&format!("{} {}", INPUT_BASE, "pl-8"), &props.class),
                placeholder: "{props.placeholder}",
                ..props.attributes,
            }
        }
    }
}
