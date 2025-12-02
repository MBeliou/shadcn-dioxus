use dioxus::prelude::*;

use crate::{
    cn,
    separator::{Separator, SeparatorProps},
};

#[component]
pub fn ItemSeparator(#[props(into, default)] class: String, props: SeparatorProps) -> Element {
    rsx! {
        Separator {
            //"data-slot": "item-separator",
            orientation: crate::separator::SeparatorOrientation::Horizontal,
            class: cn("my-0", &class),
            ..props
        }
    }
}
