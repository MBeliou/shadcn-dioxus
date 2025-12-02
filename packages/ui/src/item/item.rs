use dioxus::prelude::*;

use crate::{cn, utils::RenderFn};

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
            // FIXME: we'll need to port tailwind variants to rust to get rid of the important (!) here.
            Self::Outline => "border-border!",
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

    //pub as_child: Option<fn(ItemChildProps, Element) -> Element>,
    pub as_child: Option<RenderFn>,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[derive(Clone)]
pub struct ItemChildProps {
    pub class: String,
    pub data_slot: &'static str,
    pub data_variant: &'static str,
    pub data_size: &'static str,
}

#[component]
pub fn Item(props: ItemProps) -> Element {
    let base_classes = item_variants(props.variant, props.size);
    let classes = cn(&base_classes, &props.class);

    // Implementing as_child is tricky in rust, we'll be using some "sane" defaults and hope for the best here.
    if let Some(render) = props.as_child {
        let child_props = ItemChildProps {
            class: classes.clone(),
            data_slot: "item",
            data_variant: props.variant.as_str(),
            data_size: props.size.as_str(),
        };

        render.0(child_props, props.children)
    } else {
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
}
