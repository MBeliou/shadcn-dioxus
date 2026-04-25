use crate::cn;
use crate::menubar::{MenubarContext, MenubarMenuContext};
use dioxus::prelude::*;

const CONTENT_BASE: &str = "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 min-w-[12rem] overflow-hidden rounded-md border p-1 shadow-md";

#[derive(Clone, PartialEq, Props)]
pub struct MenubarContentProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    let ctx = use_context::<MenubarContext>();
    let menu_ctx = use_context::<MenubarMenuContext>();
    let open_menu = ctx.open_menu;
    let set_open_menu = ctx.set_open_menu;

    let is_open = use_memo(move || open_menu() == Some(menu_ctx.id));

    if !is_open() {
        return rsx! {};
    }

    let data_state = if is_open() { "open" } else { "closed" };
    let base = cn(CONTENT_BASE, "absolute left-0 top-full mt-1");
    let classes = cn(&base, &props.class);

    rsx! {
        div {
            "data-slot": "menubar-content",
            "data-state": data_state,
            "data-side": "bottom",
            role: "menu",
            class: "{classes}",
            onmouseleave: move |_| set_open_menu.call(None),
            {props.children}
        }
    }
}
