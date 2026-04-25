use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CommandListProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = 200)]
    pub max_height: u32,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CommandList(props: CommandListProps) -> Element {
    rsx! {
        div {
            "data-slot": "command-list",
            role: "listbox",
            class: "max-h-[{props.max_height}px] overflow-y-auto overflow-x-hidden {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
