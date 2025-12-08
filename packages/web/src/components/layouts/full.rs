use crate::components::Hero;
use crate::Route;
use dioxus::prelude::*;
#[component]
pub fn FullLayout() -> Element {
    rsx! {
        Hero {}
        Outlet::<Route> {}
    }
}
