use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PopoverContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
}

#[derive(Clone, PartialEq, Props)]
pub struct PopoverProps {
    pub children: Element,
}

#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let mut open = use_signal(|| false);
    let set_open = use_callback(move |val: bool| open.set(val));

    use_context_provider(|| PopoverContext { open, set_open });

    rsx! {
        div { class: "relative inline-block text-left", {props.children} }
    }
}
