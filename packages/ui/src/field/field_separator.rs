use crate::{cn, Separator};
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct FieldSeparatorProps {
    #[props(into, default)]
    pub class: String,
    #[props(default)]
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn FieldSeparator(props: FieldSeparatorProps) -> Element {
    let has_content = props.children.is_ok();
    rsx! {
        div {
            "data-slot": "field-separator",
            "data-content": has_content,
            class: cn(
                "relative -my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2",
                &props.class,
            ),
            ..props.attributes,
            Separator { class: "absolute inset-0 top-1/2" }
            if has_content {
                span {
                    "data-slot": "field-separator-content",
                    class: "bg-background text-muted-foreground relative mx-auto block w-fit px-2",
                    {props.children}
                }
            }
        }
    }
}
