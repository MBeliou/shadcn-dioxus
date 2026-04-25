use crate::cn;
use crate::select::SelectContext;
use dioxus::prelude::*;
use lucide_dioxus::ChevronDown;

const TRIGGER_BASE: &str = "flex h-9 w-full items-center justify-between whitespace-nowrap rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";

#[derive(Clone, PartialEq, Props)]
pub struct SelectTriggerProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let ctx = use_context::<SelectContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    let chevron_class = if open() {
        "h-4 w-4 opacity-50 transition-transform duration-200 rotate-180"
    } else {
        "h-4 w-4 opacity-50 transition-transform duration-200"
    };

    let classes = cn(TRIGGER_BASE, &props.class);

    rsx! {
        button {
            r#type: "button",
            "data-slot": "select-trigger",
            "aria-expanded": open(),
            class: "{classes}",
            onclick: move |_| set_open.call(!open()),
            ..props.attributes,
            {props.children}
            ChevronDown { class: chevron_class }
        }
    }
}
