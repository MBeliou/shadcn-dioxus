use crate::cn;
use dioxus::prelude::*;

const LIST_BASE: &str = "bg-muted text-muted-foreground inline-flex h-9 items-center justify-center rounded-lg p-1";

#[derive(Clone, PartialEq, Props)]
pub struct TabsListProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TabsList(props: TabsListProps) -> Element {
    rsx! {
        div {
            "data-slot": "tabs-list",
            role: "tablist",
            class: cn(LIST_BASE, &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
