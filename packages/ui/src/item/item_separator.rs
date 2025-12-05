use dioxus::prelude::*;
use crate::{cn, separator::Separator};
#[derive(Props, Clone, PartialEq)]
pub struct ItemSeparatorProps {
    #[props(into, default)]
    class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn ItemSeparator(props: ItemSeparatorProps) -> Element {
    rsx! {
        Separator {
            "data-slot": "item-separator",
            orientation: crate::separator::SeparatorOrientation::Horizontal,
            class: cn("my-0", &props.class),
            attributes: props.attributes,
        }
    }
}
