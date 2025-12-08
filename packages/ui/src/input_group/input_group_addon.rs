use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum InputGroupAddonAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

impl InputGroupAddonAlign {
    pub fn class(&self) -> &'static str {
        match self {
            Self::InlineStart => {
                "order-first ps-3 has-[>button]:ms-[-0.45rem] has-[>kbd]:ms-[-0.35rem]"
            }
            Self::InlineEnd => {
                "order-last pe-3 has-[>button]:me-[-0.45rem] has-[>kbd]:me-[-0.35rem]"
            }
            Self::BlockStart => {
                "[.border-b]:pb-3 order-first w-full justify-start px-3 pt-3 group-has-[>input]/input-group:pt-2.5"
            }
            Self::BlockEnd => {
                "[.border-t]:pt-3 order-last w-full justify-start px-3 pb-3 group-has-[>input]/input-group:pb-2.5"
            }
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InlineStart => "inline-start",
            Self::InlineEnd => "inline-end",
            Self::BlockStart => "block-start",
            Self::BlockEnd => "block-end",
        }
    }
}

const BASE_CLASSES: &str = "text-muted-foreground flex h-auto cursor-text select-none items-center justify-center gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&>kbd]:rounded-[calc(var(--radius)-5px)] [&>svg:not([class*='size-'])]:size-4";

pub fn input_group_addon_variants(align: InputGroupAddonAlign) -> String {
    format!("{} {}", BASE_CLASSES, align.class())
}

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupAddonProps {
    #[props(default)]
    pub align: InputGroupAddonAlign,

    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// TODO: The original version focuses the sibling input when the addon is clicked.
/// This is not implemented for cross-platform compatibility.
#[component]
pub fn InputGroupAddon(props: InputGroupAddonProps) -> Element {
    rsx! {
        div {
            role: "group",
            "data-slot": "input-group-addon",
            "data-align": props.align.as_str(),
            class: cn(&input_group_addon_variants(props.align), &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
