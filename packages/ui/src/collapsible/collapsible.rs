use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CollapsibleContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
}

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleProps {
    #[props(default = false)]
    pub default_open: bool,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    let mut open = use_signal(|| props.default_open);
    let set_open = use_callback(move |val: bool| open.set(val));

    use_context_provider(|| CollapsibleContext { open, set_open });

    let data_state = if open() { "open" } else { "closed" };

    rsx! {
        div {
            "data-slot": "collapsible",
            "data-state": data_state,
            class: "{props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
