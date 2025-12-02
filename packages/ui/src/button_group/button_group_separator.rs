use dioxus::prelude::*;

use crate::{cn, separator::SeparatorOrientation, Separator};

const SEPARATOR_BASE: &str = "bg-border shrink-0";
const BUTTON_GROUP_SEPARATOR_CLASSES: &str =
    "bg-input relative !m-0 self-stretch data-[orientation=vertical]:h-auto";

#[component]
pub fn ButtonGroupSeparator(
    #[props(default)] orientation: SeparatorOrientation,
    #[props(into, default)] class: String,
    #[props(default = true)] decorative: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        Separator{
            "data-slot": "button-group-separator",
            orientation: orientation,
            class: cn(BUTTON_GROUP_SEPARATOR_CLASSES, &class),
            attributes
        }
    }
}
