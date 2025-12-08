use crate::{cn, ButtonVariant};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum InputGroupButtonSize {
    #[default]
    Xs,
    Sm,
    IconXs,
    IconSm,
}

impl InputGroupButtonSize {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Xs => "h-6 gap-1 rounded-[calc(var(--radius)-5px)] px-2 has-[>svg]:px-2 [&>svg:not([class*='size-'])]:size-3.5",
            Self::Sm => "h-8 gap-1.5 rounded-md px-2.5 has-[>svg]:px-2.5",
            Self::IconXs => "size-6 rounded-[calc(var(--radius)-5px)] p-0 has-[>svg]:p-0",
            Self::IconSm => "size-8 p-0 has-[>svg]:p-0",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::IconXs => "icon-xs",
            Self::IconSm => "icon-sm",
        }
    }
}

const BASE_CLASSES: &str = "flex items-center gap-2 text-sm shadow-none";

pub fn input_group_button_variants(size: InputGroupButtonSize) -> String {
    format!("{} {}", BASE_CLASSES, size.class())
}

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupButtonProps {
    #[props(default)]
    pub size: InputGroupButtonSize,

    #[props(default = ButtonVariant::Ghost)]
    pub variant: ButtonVariant,

    #[props(into, default)]
    pub class: String,

    #[props(default = false)]
    pub disabled: bool,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn InputGroupButton(props: InputGroupButtonProps) -> Element {
    let size_classes = input_group_button_variants(props.size);
    let variant_classes = props.variant.class();

    rsx! {
        button {
            r#type: "button",
            "data-size": props.size.as_str(),
            disabled: props.disabled,
            class: cn(&format!("{} {}", size_classes, variant_classes), &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
