use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TabsContext {
    pub value: Signal<String>,
    pub set_value: Callback<String>,
}

#[derive(Clone, PartialEq, Props)]
pub struct TabsProps {
    #[props(into, default)]
    pub default_value: String,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let mut value = use_signal(|| props.default_value);
    let set_value = use_callback(move |val: String| value.set(val));

    use_context_provider(|| TabsContext { value, set_value });

    rsx! {
        div {
            "data-slot": "tabs",
            class: "{props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
