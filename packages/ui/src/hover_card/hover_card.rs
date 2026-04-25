use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HoverCardContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardProps {
    pub children: Element,
}

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    let mut open = use_signal(|| false);
    let set_open = use_callback(move |val: bool| open.set(val));

    use_context_provider(|| HoverCardContext { open, set_open });

    rsx! {
        div { class: "relative inline-block", {props.children} }
    }
}
