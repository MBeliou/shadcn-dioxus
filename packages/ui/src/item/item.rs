use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

impl ItemVariant {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "bg-transparent",
            Self::Outline => "border-border",
            Self::Muted => "bg-muted/50",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
            Self::Muted => "muted",
        }
    }
}

impl ItemSize {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "gap-4 p-4",
            Self::Sm => "gap-2.5 px-4 py-3",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

const BASE_CLASSES: &str = "group/item [a]:hover:bg-accent/50 [a]:transition-colors focus-visible:border-ring focus-visible:ring-ring/50 flex flex-wrap items-center rounded-md border border-transparent text-sm outline-none transition-colors duration-100 focus-visible:ring-[3px]";

pub fn item_variants(variant: ItemVariant, size: ItemSize) -> String {
    format!("{} {} {}", BASE_CLASSES, variant.class(), size.class())
}

#[derive(Clone, PartialEq, Props)]
pub struct ItemProps {
    #[props(default)]
    pub variant: ItemVariant,

    #[props(default)]
    pub size: ItemSize,

    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Item(props: ItemProps) -> Element {
    let base_classes = item_variants(props.variant, props.size);
    let classes = cn(&base_classes, &props.class);

    rsx! {
        div {
            "data-slot": "item",
            "data-variant": props.variant.as_str(),
            "data-size": props.size.as_str(),
            class: "{classes}",
            ..props.attributes,
            {props.children}
        }
    }
}