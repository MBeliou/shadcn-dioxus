use crate::cn;
use crate::accordion::{AccordionContext, AccordionItemContext};
use dioxus::prelude::*;
use lucide_dioxus::ChevronDown;

const TRIGGER_BASE: &str = "flex flex-1 items-center justify-between py-4 text-sm font-medium transition-all hover:underline text-left [&[data-state=open]>svg]:rotate-180";

#[derive(Clone, PartialEq, Props)]
pub struct AccordionTriggerProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    let ctx = use_context::<AccordionContext>();
    let item_ctx = use_context::<AccordionItemContext>();
    let open_items = ctx.open_items;
    let set_open_items = ctx.set_open_items;
    let multiple = ctx.multiple;
    let value = item_ctx.value.clone();
    let value_for_memo = item_ctx.value.clone();

    let is_open = use_memo(move || (open_items)().contains(&value_for_memo));
    let data_state = if is_open() { "open" } else { "closed" };
    let classes = cn(TRIGGER_BASE, &props.class);

    rsx! {
        h3 { class: "flex",
            button {
                r#type: "button",
                "data-slot": "accordion-trigger",
                "data-state": data_state,
                "aria-expanded": is_open(),
                class: "{classes}",
                onclick: move |_| {
                    let current = (open_items)();
                    let new_items = if is_open() {
                        current.into_iter().filter(|v| v != &value).collect()
                    } else if multiple {
                        let mut new = current;
                        new.push(value.clone());
                        new
                    } else {
                        vec![value.clone()]
                    };
                    set_open_items.call(new_items);
                },
                ..props.attributes,
                {props.children}
                ChevronDown { class: "h-4 w-4 shrink-0 text-muted-foreground transition-transform duration-200" }
            }
        }
    }
}
