use crate::cn;
use crate::tabs::TabsContext;
use dioxus::prelude::*;

const TRIGGER_BASE: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow";

#[derive(Clone, PartialEq, Props)]
pub struct TabsTriggerProps {
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
pub fn TabsTrigger(props: TabsTriggerProps) -> Element {
    let ctx = use_context::<TabsContext>();
    let current_value = ctx.value;
    let set_value = ctx.set_value;
    let value = props.value.clone();

    let is_active = use_memo(move || current_value() == value);
    let data_state = if is_active() { "active" } else { "inactive" };
    let classes = cn(TRIGGER_BASE, &props.class);

    rsx! {
        button {
            r#type: "button",
            "data-slot": "tabs-trigger",
            role: "tab",
            "data-state": data_state,
            "aria-selected": is_active(),
            disabled: props.disabled,
            class: "{classes}",
            onclick: move |_| set_value.call(props.value.clone()),
            ..props.attributes,
            {props.children}
        }
    }
}
