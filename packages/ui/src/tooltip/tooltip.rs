use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TooltipContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
}

#[derive(Clone, PartialEq, Props)]
pub struct TooltipProps {
    #[props(default = 0)]
    pub delay_duration: u64,
    pub children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let mut open = use_signal(|| false);
    let set_open = use_callback(move |val: bool| open.set(val));

    use_context_provider(|| TooltipContext { open, set_open });

    rsx! {
        div { class: "relative inline-block", {props.children} }
    }
}
