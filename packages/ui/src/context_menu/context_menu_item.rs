use crate::cn;
use dioxus::prelude::*;

const ITEM_BASE: &str = "relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&>svg]:size-4 [&>svg]:shrink-0";

#[derive(Clone, PartialEq, Props)]
pub struct ContextMenuItemProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    rsx! {
        div {
            "data-slot": "context-menu-item",
            role: "menuitem",
            tabindex: "0",
            class: cn(ITEM_BASE, &props.class),
            "data-disabled": if props.disabled { "true" } else { "false" },
            onclick: move |evt| {
                if let Some(cb) = &props.onclick {
                    cb.call(evt);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}
