use crate::cn;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct FieldSetProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn FieldSet(props: FieldSetProps) -> Element {
    rsx! {
        fieldset {
            "data-slot": "field-set",
            class: cn(
                "flex flex-col gap-6 has-[>[data-slot=checkbox-group]]:gap-3 has-[>[data-slot=radio-group]]:gap-3",
                &props.class,
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
