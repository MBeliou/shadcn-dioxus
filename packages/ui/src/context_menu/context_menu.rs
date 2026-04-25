use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ContextMenuContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
}

#[derive(Clone, PartialEq, Props)]
pub struct ContextMenuProps {
    pub children: Element,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    let mut open = use_signal(|| false);
    let set_open = use_callback(move |val: bool| open.set(val));

    use_context_provider(|| ContextMenuContext { open, set_open });

    rsx! {
        div { class: "relative inline-block", {props.children} }
    }
}
