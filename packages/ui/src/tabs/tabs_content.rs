use crate::cn;
use crate::tabs::TabsContext;
use dioxus::prelude::*;

const CONTENT_BASE: &str = "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2";

#[derive(Clone, PartialEq, Props)]
pub struct TabsContentProps {
    #[props(into)]
    pub value: String,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TabsContent(props: TabsContentProps) -> Element {
    let ctx = use_context::<TabsContext>();
    let current_value = ctx.value;

    let is_active = use_memo(move || current_value() == props.value);

    if !is_active() {
        return rsx! {};
    }

    let classes = cn(CONTENT_BASE, &props.class);

    rsx! {
        div {
            "data-slot": "tabs-content",
            role: "tabpanel",
            "data-state": "active",
            class: "{classes}",
            ..props.attributes,
            {props.children}
        }
    }
}
