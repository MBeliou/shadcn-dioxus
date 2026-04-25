use dioxus::prelude::*;
use crate::select::SelectContext;

#[derive(Clone, PartialEq, Props)]
pub struct SelectValueProps {
    #[props(default = "".to_string())]
    pub placeholder: String,
    pub children: Element,
}

#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    let ctx = use_context::<SelectContext>();
    let value = ctx.value;

    rsx! {
        span {
            "data-slot": "select-value",
            if value().is_empty() {
                span { class: "text-muted-foreground", "{props.placeholder}" }
            } else {
                {props.children}
            }
        }
    }
}
