use dioxus::prelude::*;
use crate::cn;


#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ButtonGroupOrientation {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Horizontal => "[&>*:not(:first-child)]:rounded-s-none [&>*:not(:first-child)]:border-s-0 [&>*:not(:last-child)]:rounded-e-none",
            Self::Vertical => "flex-col [&>*:not(:first-child)]:rounded-t-none [&>*:not(:first-child)]:border-t-0 [&>*:not(:last-child)]:rounded-b-none",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

const BASE_CLASSES: &str = "flex w-fit items-stretch has-[>[data-slot=button-group]]:gap-2 [&>*]:focus-visible:relative [&>*]:focus-visible:z-10 has-[select[aria-hidden=true]:last-child]:[&>[data-slot=select-trigger]:last-of-type]:rounded-e-md [&>[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&>input]:flex-1";

pub fn button_group_variants(orientation: ButtonGroupOrientation) -> String {
    format!("{} {}", BASE_CLASSES, orientation.class())
}
/* 
fn cn(base: &str, additional: Option<&str>) -> String {
    match additional {
        Some(extra) if !extra.is_empty() => format!("{} {}", base, extra),
        _ => base.to_string(),
    }
}*/

#[component]
pub fn ButtonGroup(
    #[props(default)] orientation: ButtonGroupOrientation,
    class: Option<String>,
    children: Element,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base_classes = button_group_variants(orientation);
    let classes = cn(&base_classes, class.as_deref());

    rsx! {
        div {
            role: "group",
            "data-slot": "button-group",
            "data-orientation": orientation.as_str(),
            class: "{classes}",
            ..attributes,
            {children}
        }
    }
}