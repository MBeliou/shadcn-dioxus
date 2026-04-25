use crate::cn;
use crate::drawer::DrawerContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DrawerCloseProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn DrawerClose(props: DrawerCloseProps) -> Element {
    let ctx = use_context::<DrawerContext>();
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "drawer-close",
            class: cn("", &props.class),
            onclick: move |_| set_open.call(false),
            {props.children}
        }
    }
}
