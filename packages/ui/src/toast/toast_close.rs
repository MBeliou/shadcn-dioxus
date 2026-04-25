use crate::cn;
use super::toast_provider::remove_toast;
use dioxus::prelude::*;
use lucide_dioxus::X;

#[derive(Clone, PartialEq, Props)]
pub struct ToastCloseProps {
    pub id: usize,
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn ToastClose(props: ToastCloseProps) -> Element {
    let id = props.id;
    let classes = cn(
        "absolute right-1 top-1 rounded-md p-1 text-foreground/50 opacity-0 transition-opacity hover:text-foreground focus:outline-none focus:ring-1",
        &props.class,
    );

    rsx! {
        button {
            r#type: "button",
            "data-slot": "toast-close",
            class: "{classes}",
            "aria-label": "Close",
            onclick: move |_| remove_toast(id),
            X { class: "h-4 w-4" }
            span { class: "sr-only", "Close" }
        }
    }
}
