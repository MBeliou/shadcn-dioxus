use crate::cn;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct KbdProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Kbd(props: KbdProps) -> Element {
    rsx! {
        kbd {
            "data-slot": "kbd",
            class: cn(
                "pointer-events-none inline-flex h-5 w-fit min-w-5 select-none items-center justify-center gap-1 rounded-sm border border-border bg-muted px-1 font-mono text-xs font-medium text-foreground shadow-sm",
                &props.class,
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
