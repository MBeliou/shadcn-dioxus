use crate::cn;
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

impl FieldOrientation {
    fn class(&self) -> &'static str {
        match self {
            Self::Vertical => "flex-col [&>*]:w-full [&>.sr-only]:w-auto",
            Self::Horizontal => "flex-row items-center [&>[data-slot=field-label]]:flex-auto has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px has-[>[data-slot=field-content]]:items-start",
            Self::Responsive => "@md/field-group:flex-row @md/field-group:items-center @md/field-group:[&>*]:w-auto flex-col [&>*]:w-full [&>.sr-only]:w-auto @md/field-group:[&>[data-slot=field-label]]:flex-auto @md/field-group:has-[>[data-slot=field-content]]:items-start @md/field-group:has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FieldProps {
    #[props(default)]
    pub orientation: FieldOrientation,

    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Field(props: FieldProps) -> Element {
    rsx! {
        div {
            role: "group",
            "data-slot": "field",
            "data-orientation": match props.orientation {
                FieldOrientation::Vertical => "vertical",
                FieldOrientation::Horizontal => "horizontal",
                FieldOrientation::Responsive => "responsive",
            },
            class: cn(
                &format!("group/field data-[invalid=true]:text-destructive flex w-full gap-3 {}", props.orientation.class()),
                &props.class
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
