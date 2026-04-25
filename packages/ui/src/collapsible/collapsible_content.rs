use crate::cn;
use crate::collapsible::CollapsibleContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleContentProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    let ctx = use_context::<CollapsibleContext>();
    let open = ctx.open;

    // grid-rows transition approach – no extra CSS keyframes needed
    let grid_rows = if open() { "1fr" } else { "0fr" };

    rsx! {
        div {
            "data-slot": "collapsible-content",
            class: cn(
                "grid transition-[grid-template-rows] duration-300 ease-out",
                &props.class,
            ),
            style: "grid-template-rows: {grid_rows};",
            ..props.attributes,
            div {
                class: "overflow-hidden",
                {props.children}
            }
        }
    }
}
