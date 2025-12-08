use crate::cn;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FieldDescriptionProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn FieldDescription(props: FieldDescriptionProps) -> Element {
    rsx! {
        p {
            "data-slot": "field-description",
            class: cn(
                "text-muted-foreground text-sm font-normal leading-normal group-has-[[data-orientation=horizontal]]/field:text-balance nth-last-2:-mt-1 last:mt-0 [[data-variant=legend]+&]:-mt-1.5 [&>a:hover]:text-primary [&>a]:underline [&>a]:underline-offset-4",
                &props.class
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
