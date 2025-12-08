use crate::cn;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FieldGroupProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn FieldGroup(props: FieldGroupProps) -> Element {
    rsx! {
        div {
            "data-slot": "field-group",
            class: cn(
                "group/field-group @container/field-group flex w-full flex-col gap-7 data-[slot=checkbox-group]:gap-3 [&>[data-slot=field-group]]:gap-4",
                &props.class
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
