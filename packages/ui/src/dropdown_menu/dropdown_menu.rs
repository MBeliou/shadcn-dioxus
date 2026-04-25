use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DropdownMenuContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
}

#[derive(Clone, PartialEq, Props)]
pub struct DropdownMenuProps {
    pub children: Element,
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let mut open = use_signal(|| false);
    let set_open = use_callback(move |val: bool| open.set(val));

    use_context_provider(|| DropdownMenuContext { open, set_open });

    rsx! {
        div { class: "relative inline-block text-left", {props.children} }
    }
}
