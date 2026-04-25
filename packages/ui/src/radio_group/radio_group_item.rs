use crate::cn;
use crate::radio_group::RadioGroupContext;
use dioxus::prelude::*;

const ITEM_BASE: &str = "aspect-square h-4 w-4 rounded-full border border-primary text-primary shadow focus:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct RadioGroupItemProps {
    #[props(into)]
    pub value: String,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn RadioGroupItem(props: RadioGroupItemProps) -> Element {
    let ctx = use_context::<RadioGroupContext>();
    let current_value = ctx.value;
    let set_value = ctx.set_value;
    let value = props.value.clone();
    let value_for_memo = props.value.clone();

    let is_checked = use_memo(move || (current_value)() == Some(value_for_memo.clone()));

    rsx! {
        div { class: "flex items-center space-x-2",
            button {
                r#type: "button",
                role: "radio",
                "data-slot": "radio-group-item",
                "aria-checked": is_checked(),
                disabled: props.disabled,
                class: cn(ITEM_BASE, &props.class),
                onclick: move |_| set_value.call(Some(value.clone())),
                ..props.attributes,
                if is_checked() {
                    span { class: "flex items-center justify-center",
                        span { class: "h-2.5 w-2.5 rounded-full bg-current" }
                    }
                }
            }
        }
    }
}
