use crate::cn;
use crate::accordion::AccordionContext;
use dioxus::prelude::*;
use super::accordion_item::AccordionItemContext;

const CONTENT_BASE: &str = "data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down overflow-hidden text-sm transition-all";

#[derive(Clone, PartialEq, Props)]
pub struct AccordionContentProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let ctx = use_context::<AccordionContext>();
    let item_ctx = use_context::<AccordionItemContext>();
    let open_items = ctx.open_items;
    let value = item_ctx.value.clone();

    let is_open = use_memo(move || open_items().contains(&value));

    if !is_open() {
        return rsx! {};
    }

    let data_state = if is_open() { "open" } else { "closed" };
    let base = cn(CONTENT_BASE, "pb-4 pt-0");
    let classes = cn(&base, &props.class);

    rsx! {
        div {
            "data-slot": "accordion-content",
            "data-state": data_state,
            class: "{classes}",
            ..props.attributes,
            {props.children}
        }
    }
}
