use crate::cn;
use crate::menubar::{MenubarContext, MenubarMenuContext};
use dioxus::prelude::*;

const TRIGGER_BASE: &str = "flex cursor-default select-none items-center rounded-sm px-3 py-1.5 text-sm font-medium outline-none focus:bg-accent focus:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground";

#[derive(Clone, PartialEq, Props)]
pub struct MenubarTriggerProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let ctx = use_context::<MenubarContext>();
    let menu_ctx = use_context::<MenubarMenuContext>();
    let open_menu = ctx.open_menu;
    let set_open_menu = ctx.set_open_menu;

    let is_open = use_memo(move || open_menu() == Some(menu_ctx.id));

    rsx! {
        button {
            r#type: "button",
            "data-slot": "menubar-trigger",
            "data-state": if is_open() { "open" } else { "closed" },
            class: cn(TRIGGER_BASE, &props.class),
            onclick: move |_| {
                let new = if is_open() { None } else { Some(menu_ctx.id) };
                set_open_menu.call(new);
            },
            onmouseenter: move |_| set_open_menu.call(Some(menu_ctx.id)),
            {props.children}
        }
    }
}
