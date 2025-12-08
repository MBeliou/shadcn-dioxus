use crate::cn;
use dioxus::prelude::*;
#[derive(Clone, Copy, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}
impl AlertVariant {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "bg-card text-card-foreground",
            Self::Destructive => {
                "text-destructive bg-card *:data-[slot=alert-description]:text-destructive/90 [&>svg]:text-current"
            }
        }
    }
}
const ALERT_BASE: &str = "relative grid w-full grid-cols-[0_1fr] items-start gap-y-0.5 rounded-lg border px-4 py-3 text-sm has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] has-[>svg]:gap-x-3 [&>svg]:size-4 [&>svg]:translate-y-0.5 [&>svg]:text-current";
#[derive(Clone, PartialEq, Props)]
pub struct AlertProps {
    #[props(default)]
    pub variant: AlertVariant,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let base_classes = cn(ALERT_BASE, props.variant.class());
    let classes = cn(&base_classes, &props.class);
    rsx! {
        div { "data-slot": "alert", class: "{classes}", ..props.attributes, {props.children} }
    }
}
