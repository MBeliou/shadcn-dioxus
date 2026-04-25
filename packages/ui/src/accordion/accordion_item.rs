use dioxus::prelude::*;
use super::accordion::AccordionContext;

#[derive(Clone, PartialEq)]
pub struct AccordionItemContext {
    pub value: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionItemProps {
    #[props(into)]
    pub value: String,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let ctx = use_context::<AccordionContext>();
    let value_for_memo = props.value.clone();
    let is_open = use_memo(move || (ctx.open_items)().contains(&value_for_memo));
    let data_state = if is_open() { "open" } else { "closed" };

    use_context_provider(|| AccordionItemContext { value: props.value.clone() });

    rsx! {
        div {
            "data-slot": "accordion-item",
            "data-state": data_state,
            class: "border-b {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
