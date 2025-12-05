use dioxus::prelude::*;

use crate::cn;

#[component]
pub fn ItemGroup(
    #[props(into, default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            role: "list",
            "data-slot": "item-group",
            class: cn("group/item-group flex flex-col", &class),
            {children}
        }
    }
}
