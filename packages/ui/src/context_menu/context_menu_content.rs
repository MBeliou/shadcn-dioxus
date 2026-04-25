use crate::cn;
use crate::context_menu::ContextMenuContext;
use dioxus::prelude::*;

const CONTENT_BASE: &str = "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 shadow-md";

#[derive(Clone, PartialEq, Props)]
pub struct ContextMenuContentProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub force_mount: bool,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    let ctx = use_context::<ContextMenuContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    if !props.force_mount && !open() {
        return rsx! {};
    }

    let data_state = if open() { "open" } else { "closed" };
    let classes = cn(CONTENT_BASE, "absolute top-0 left-0");
    let final_classes = cn(&classes, &props.class);

    rsx! {
        div {
            class: "fixed inset-0 z-40",
            onclick: move |_| set_open.call(false),
        }
        div {
            "data-slot": "context-menu-content",
            "data-state": data_state,
            role: "menu",
            class: "{final_classes}",
            onclick: move |evt| evt.stop_propagation(),
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Escape {
                    set_open.call(false);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}
