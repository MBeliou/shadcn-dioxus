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
                div { class: "w-full max-w-2xl mx-auto py-6 px-4 md:px-0 lg:py-8", Outlet::<Route> {} }
            }
        }
    }
}
