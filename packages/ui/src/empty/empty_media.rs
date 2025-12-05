use dioxus::prelude::*;
use crate::cn;
#[derive(Clone, Copy, PartialEq, Default)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
}
impl EmptyMediaVariant {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "bg-transparent",
            Self::Icon => {
                "bg-muted text-foreground flex size-10 shrink-0 items-center justify-center rounded-lg [&_svg:not([class*='size-'])]:size-6"
            }
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Icon => "icon",
        }
    }
}
const EMPTY_MEDIA_BASE: &str = "mb-2 flex shrink-0 items-center justify-center [&_svg]:pointer-events-none [&_svg]:shrink-0";
pub fn empty_media_variants(variant: EmptyMediaVariant) -> String {
    format!("{} {}", EMPTY_MEDIA_BASE, variant.class())
}
#[derive(Clone, PartialEq, Props)]
pub struct EmptyMediaProps {
    #[props(default)]
    pub variant: EmptyMediaVariant,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn EmptyMedia(props: EmptyMediaProps) -> Element {
    let base_classes = empty_media_variants(props.variant);
    let classes = cn(&base_classes, &props.class);
    rsx! {
        div {
            "data-slot": "empty-icon",
            "data-variant": props.variant.as_str(),
            class: "{classes}",
            ..props.attributes,
            {props.children}
        }
    }
}
