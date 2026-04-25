use crate::cn;
use dioxus::prelude::*;

const ITEM_BASE: &str = "relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&>svg]:size-4 [&>svg]:shrink-0";

#[derive(Clone, PartialEq, Props)]
pub struct MenubarItemProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    let classes = cn(ITEM_BASE, &props.class);

    rsx! {
        div {
            "data-slot": "menubar-item",
            role: "menuitem",
            tabindex: "0",
            class: "{classes}",
            "data-disabled": if props.disabled { "true" } else { "false" },
            onclick: move |evt| {
                if !props.disabled {
                    if let Some(cb) = &props.onclick {
                        cb.call(evt);
                    }
                }
            },
            {props.children}
        }
    }
}
