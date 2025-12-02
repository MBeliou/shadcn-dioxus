use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "flex flex-1 w-screen",
            div { "sidebar" }
            div { class: "flex-1",
                div { class: "mx-auto flex w-full max-w-2xl", Outlet::<crate::Route> {} }
            }
        }
    }
}
