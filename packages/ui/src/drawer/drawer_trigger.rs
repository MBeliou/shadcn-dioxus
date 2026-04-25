use crate::drawer::DrawerContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DrawerTriggerProps {
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DrawerTrigger(props: DrawerTriggerProps) -> Element {
    let ctx = use_context::<DrawerContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "drawer-trigger",
            "aria-expanded": open(),
            onclick: move |_| set_open.call(!open()),
            ..props.attributes,
            {props.children}
        }
    }
}
