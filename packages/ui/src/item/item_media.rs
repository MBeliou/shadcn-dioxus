use dioxus::prelude::*;
use crate::cn;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "bg-transparent",
            Self::Icon => "size-8 border rounded-sm bg-muted [&_svg:not([class*='size-'])]:size-4",
            Self::Image => "size-10 rounded-sm overflow-hidden [&_img]:size-full [&_img]:object-cover",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Icon => "icon",
            Self::Image => "image",
        }
    }
}

const ITEM_MEDIA_BASE: &str = "flex shrink-0 items-center justify-center gap-2 group-has-[[data-slot=item-description]]/item:self-start [&_svg]:pointer-events-none group-has-[[data-slot=item-description]]/item:translate-y-0.5";

pub fn item_media_variants(variant: ItemMediaVariant) -> String {
    format!("{} {}", ITEM_MEDIA_BASE, variant.class())
}

#[derive(Clone, PartialEq, Props)]
pub struct ItemMediaProps {
    #[props(default)]
    pub variant: ItemMediaVariant,

    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ItemMedia(props: ItemMediaProps) -> Element {
    let base_classes = item_media_variants(props.variant);
    let classes = cn(&base_classes, &props.class);

    rsx! {
        div {
            "data-slot": "item-media",
            "data-variant": props.variant.as_str(),
            class: "{classes}",
            ..props.attributes,
            {props.children}
        }
    }
}