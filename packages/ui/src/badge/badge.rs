use crate::cn;
use dioxus::prelude::*;
#[derive(Clone, Copy, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}
impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => {
                "bg-primary text-primary-foreground [a&]:hover:bg-primary/90 border-transparent"
            }
            Self::Secondary => {
                "bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90 border-transparent"
            }
            Self::Destructive => {
                "bg-destructive [a&]:hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/70 border-transparent text-white"
            }
            Self::Outline => {
                "text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground"
            }
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Secondary => "secondary",
            Self::Destructive => "destructive",
            Self::Outline => "outline",
        }
    }
}
const BADGE_BASE: &str = "focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex w-fit shrink-0 items-center justify-center gap-1 overflow-hidden whitespace-nowrap rounded-full border px-2 py-0.5 text-xs font-medium transition-[color,box-shadow] focus-visible:ring-[3px] [&>svg]:pointer-events-none [&>svg]:size-3";
pub fn badge_variants(variant: BadgeVariant) -> String {
    format!("{} {}", BADGE_BASE, variant.class())
}
#[derive(Clone, PartialEq, Props)]
pub struct BadgeProps {
    #[props(default)]
    pub variant: BadgeVariant,
    #[props(into, default)]
    pub class: String,
    pub href: Option<String>,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let base_classes = badge_variants(props.variant);
    let classes = cn(&base_classes, &props.class);
    if let Some(href_val) = props.href {
        rsx! {
            a {
                "data-slot": "badge",
                href: "{href_val}",
                class: "{classes}",
                ..props.attributes,
                {props.children}
            }
        }
    } else {
        rsx! {
            span { "data-slot": "badge", class: "{classes}", ..props.attributes, {props.children} }
        }
    }
}
