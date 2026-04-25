use crate::dropdown_menu::DropdownMenuContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DropdownMenuTriggerProps {
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let ctx = use_context::<DropdownMenuContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "dropdown-menu-trigger",
            "aria-expanded": open(),
            "aria-haspopup": "menu",
            onclick: move |_| set_open.call(!open()),
            ..props.attributes,
            {props.children}
        }
    }
}
