use crate::{cn, Label};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FieldLabelProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn FieldLabel(props: FieldLabelProps) -> Element {
    rsx! {
        Label {
            "data-slot": "field-label",
            class: cn(
                "group/field-label peer/field-label flex w-fit gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[>[data-slot=field]]:w-full has-[>[data-slot=field]]:flex-col has-[>[data-slot=field]]:rounded-md has-[>[data-slot=field]]:border [&>*]:data-[slot=field]:p-4 has-data-[state=checked]:bg-primary/5 has-data-[state=checked]:border-primary dark:has-data-[state=checked]:bg-primary/10",
                &props.class
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
