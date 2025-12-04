use super::Sidebar;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    let route = use_route::<Route>();

    let active_slug = match route {
        Route::ComponentDoc { name } => name,
        _ => String::new(),
    };

    rsx! {
        div { class: "flex flex-1 w-screen",
            Sidebar { active_slug }
            div { class: "flex-1",
                div { class: "mx-auto flex w-full max-w-2xl py-8 px-4", Outlet::<Route> {} }
            }
        }
    }
}
