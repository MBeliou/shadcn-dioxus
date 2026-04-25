use crate::cn;
use crate::select::SelectContext;
use dioxus::prelude::*;

const ITEM_BASE: &str = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-2 pr-8 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct SelectItemProps {
    #[props(into)]
    pub value: String,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SelectItem(props: SelectItemProps) -> Element {
    let ctx = use_context::<SelectContext>();
    let current_value = ctx.value;
    let set_value = ctx.set_value;
    let set_open = ctx.set_open;
    let value = props.value.clone();
    let value_for_memo = props.value.clone();

    let is_selected = use_memo(move || (current_value)() == value_for_memo);
    let classes = cn(ITEM_BASE, &props.class);

    rsx! {
        div {
            "data-slot": "select-item",
            role: "option",
            "aria-selected": is_selected(),
            "data-disabled": if props.disabled { "true" } else { "false" },
            class: "{classes}",
            onclick: move |_| {
                if !props.disabled {
                    set_value.call(value.clone());
                    set_open.call(false);
                }
            },
            ..props.attributes,
            {props.children}
            if is_selected() {
                span { class: "absolute right-2 flex h-3.5 w-3.5 items-center justify-center",
                    lucide_dioxus::Check { class: "h-4 w-4" }
                }
            }
        }
    }
}
