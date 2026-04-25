use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ToggleGroupType {
    #[default]
    Single,
    Multiple,
}

impl ToggleGroupType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ToggleGroupContext {
    pub value: Signal<Vec<String>>,
    pub set_value: Callback<Vec<String>>,
    pub group_type: ToggleGroupType,
}

#[derive(Clone, PartialEq, Props)]
pub struct ToggleGroupProps {
    #[props(default)]
    pub group_type: ToggleGroupType,
    #[props(default)]
    pub default_value: Vec<String>,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    let mut value = use_signal(|| props.default_value);
    let set_value = use_callback(move |val: Vec<String>| value.set(val));

    use_context_provider(|| ToggleGroupContext {
        value,
        set_value,
        group_type: props.group_type,
    });

    rsx! {
        div {
            "data-slot": "toggle-group",
            role: "group",
            "data-type": props.group_type.as_str(),
            class: "inline-flex items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
