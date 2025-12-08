use crate::cn;
use crate::input_group::InputGroupCtx;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupInputProps {
    #[props(into, default)]
    pub class: String,

    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn InputGroupInput(props: InputGroupInputProps) -> Element {
    let mut ctx = use_context::<InputGroupCtx>();

    rsx! {
        input {
            onmounted: move |event| {
                ctx.control.set(Some(event.data()));
            },
            "data-slot": "input-group-control",
            class: cn(
                "flex-1 rounded-none border-0 bg-transparent shadow-none focus-visible:ring-0 dark:bg-transparent",
                &props.class
            ),
            ..props.attributes,
        }
    }
}
