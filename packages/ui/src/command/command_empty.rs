use crate::command::CommandContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CommandEmptyProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CommandEmpty(props: CommandEmptyProps) -> Element {
    let ctx = use_context::<CommandContext>();
    let search = ctx.value;
    let visible = ctx.visible_count;

    // Show only when user has typed something and zero items are visible
    if search().is_empty() || visible() > 0 {
        return rsx! {};
    }

    rsx! {
        div {
            "data-slot": "command-empty",
            class: "py-6 text-center text-sm {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
