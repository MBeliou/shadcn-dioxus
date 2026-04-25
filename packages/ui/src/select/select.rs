use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SelectContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
    pub value: Signal<String>,
    pub set_value: Callback<String>,
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps {
    #[props(default)]
    pub default_value: String,
    pub children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut open = use_signal(|| false);
    let mut value = use_signal(|| props.default_value);

    let set_open = use_callback(move |val: bool| open.set(val));
    let set_value = use_callback(move |val: String| value.set(val));

    use_context_provider(|| SelectContext { open, set_open, value, set_value });

    rsx! {
        div { class: "relative inline-block w-full", {props.children} }
    }
}
