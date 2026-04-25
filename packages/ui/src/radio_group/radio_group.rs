use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RadioGroupContext {
    pub value: Signal<Option<String>>,
    pub set_value: Callback<Option<String>>,
}

#[derive(Clone, PartialEq, Props)]
pub struct RadioGroupProps {
    #[props(default)]
    pub default_value: Option<String>,
    #[props(into, default)]
    pub class: String,
    #[props(default = "radiogroup".to_string())]
    pub name: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let mut value = use_signal(|| props.default_value);
    let set_value = use_callback(move |val: Option<String>| value.set(val));

    use_context_provider(|| RadioGroupContext { value, set_value });

    rsx! {
        div {
            "data-slot": "radio-group",
            role: "radiogroup",
            "aria-label": "{props.name}",
            class: "grid gap-2 {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
