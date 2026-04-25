use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AccordionContext {
    pub open_items: Signal<Vec<String>>,
    pub set_open_items: Callback<Vec<String>>,
    pub multiple: bool,
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionProps {
    #[props(default = false)]
    pub multiple: bool,
    #[props(default)]
    pub default_value: Vec<String>,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let mut open_items = use_signal(|| props.default_value);
    let multiple = props.multiple;
    let set_open_items = use_callback(move |val: Vec<String>| open_items.set(val));

    use_context_provider(|| AccordionContext { open_items, set_open_items, multiple });

    rsx! {
        div {
            "data-slot": "accordion",
            class: "{props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
