use crate::input_group::InputGroupCtx;
use crate::Textarea;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct InputGroupTextareaProps {
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    #[props(extends = textarea)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn InputGroupTextarea(props: InputGroupTextareaProps) -> Element {
    let mut ctx = use_context::<InputGroupCtx>();
    rsx! {
        Textarea {
            onmounted: move |event: Event<MountedData>| {
                ctx.control.set(Some(event.data()));
            },
            data_slot: "input-group-control",
            class: "flex-1 resize-none rounded-none border-0 bg-transparent py-3 shadow-none focus-visible:ring-0 dark:bg-transparent {props.class}",
            attributes: props.attributes,
        }
    }
}
