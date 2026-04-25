use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AlertDialogContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
}

#[derive(Clone, PartialEq, Props)]
pub struct AlertDialogProps {
    pub children: Element,
}

#[component]
pub fn AlertDialog(props: AlertDialogProps) -> Element {
    let mut open = use_signal(|| false);
    let set_open = use_callback(move |val: bool| open.set(val));

    use_context_provider(|| AlertDialogContext { open, set_open });

    rsx! {
        {props.children}
    }
}
