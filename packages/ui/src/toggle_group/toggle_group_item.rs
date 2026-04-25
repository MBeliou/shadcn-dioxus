use crate::cn;
use crate::toggle_group::{ToggleGroupContext, ToggleGroupType};
use dioxus::prelude::*;

const ITEM_BASE: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-background data-[state=on]:text-foreground data-[state=on]:shadow";

#[derive(Clone, PartialEq, Props)]
pub struct ToggleGroupItemProps {
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
pub fn ToggleGroupItem(props: ToggleGroupItemProps) -> Element {
    let ctx = use_context::<ToggleGroupContext>();
    let current_value = ctx.value;
    let set_value = ctx.set_value;
    let group_type = ctx.group_type;
    let value = props.value.clone();
    let value_for_memo = props.value.clone();

    let is_pressed = use_memo(move || (current_value)().contains(&value_for_memo));
    let data_state = if is_pressed() { "on" } else { "off" };
    let classes = cn(ITEM_BASE, &props.class);

    rsx! {
        button {
            r#type: "button",
            "data-slot": "toggle-group-item",
            role: "groupitem",
            "data-state": data_state,
            "aria-pressed": is_pressed(),
            disabled: props.disabled,
            class: "{classes}",
            onclick: move |_| {
                let current = (current_value)();
                let new_value = if is_pressed() {
                    current.into_iter().filter(|v| v != &value).collect()
                } else {
                    match group_type {
                        ToggleGroupType::Single => vec![value.clone()],
                        ToggleGroupType::Multiple => {
                            let mut new = current;
                            new.push(value.clone());
                            new
                        }
                    }
                };
                set_value.call(new_value);
            },
            ..props.attributes,
            {props.children}
        }
    }
}
