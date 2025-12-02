use dioxus::prelude::*;

use crate::separator::Separator;

#[component]
pub fn ItemSeparator() -> Element {
    // TODO: implement base separator
    rsx! {
        Separator {
            orientation: true,
            
        }
    }
}