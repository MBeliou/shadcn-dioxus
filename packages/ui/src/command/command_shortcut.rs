use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CommandShortcutProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CommandShortcut(props: CommandShortcutProps) -> Element {
    rsx! {
        span {
            "data-slot": "command-shortcut",
            class: cn("ml-auto text-xs tracking-widest text-muted-foreground", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
