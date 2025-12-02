use crate::components;
use dioxus::prelude::*;
#[component]
pub fn Page() -> Element {
    rsx! {
        components::Header {}
    }
}
