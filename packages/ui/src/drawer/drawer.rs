use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DrawerContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
}

#[derive(Clone, PartialEq, Props)]
pub struct DrawerProps {
    pub children: Element,
}

#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    let mut open = use_signal(|| false);
    let set_open = use_callback(move |val: bool| open.set(val));

    use_context_provider(|| DrawerContext { open, set_open });

    rsx! {
        {props.children}
    }
}
