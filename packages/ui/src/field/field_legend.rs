use crate::cn;
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FieldLegendVariant {
    #[default]
    Legend,
    Label,
}

#[derive(Props, Clone, PartialEq)]
pub struct FieldLegendProps {
    #[props(default)]
    pub variant: FieldLegendVariant,

    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn FieldLegend(props: FieldLegendProps) -> Element {
    let variant_str = match props.variant {
        FieldLegendVariant::Legend => "legend",
        FieldLegendVariant::Label => "label",
    };

    rsx! {
        legend {
            "data-slot": "field-legend",
            "data-variant": variant_str,
            class: cn(
                "mb-3 font-medium data-[variant=legend]:text-base data-[variant=label]:text-sm",
                &props.class
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
