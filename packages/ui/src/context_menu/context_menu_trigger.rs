use crate::context_menu::ContextMenuContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ContextMenuTriggerProps {
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    let set_open = use_context::<ContextMenuContext>().set_open;

    rsx! {
        div {
            "data-slot": "context-menu-trigger",
            oncontextmenu: move |evt| {
                evt.prevent_default();
                set_open.call(true);
            },
            ..props.attributes,
            {props.children}
        }
    }
}
