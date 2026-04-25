use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ToastActionProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn ToastAction(props: ToastActionProps) -> Element {
    rsx! {
        div {
            "data-slot": "toast-action",
            class: "{props.class}",
            {props.children}
        }
    }
}
