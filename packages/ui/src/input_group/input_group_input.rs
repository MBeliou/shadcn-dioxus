use crate::input_group::InputGroupCtx;
use crate::Input;
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
        Input {
            onmounted: move |element: Event<MountedData>| {
                ctx.control.set(Some(element.data()));
            },
            data_slot: "input-group-control",
            class: "flex-1 rounded-none border-0 bg-transparent shadow-none focus-visible:ring-0 dark:bg-transparent {props.class}",
            attributes: props.attributes,
        }
    }
}
