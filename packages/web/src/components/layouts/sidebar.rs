use super::super::Sidebar;
use crate::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SidebarLayoutProps {
    children: Element,
}

#[component]
pub fn SidebarLayout(props: SidebarLayoutProps) -> Element {
    let route = use_route::<Route>();
    let active_slug = match route {
        Route::ComponentDoc { name } => name,
        _ => String::new(),
    };

    rsx! {
        div { class: "flex w-full min-h-screen",
            Sidebar { active_slug }
            div { class: "flex-1",
                div { class: "w-full max-w-3xl mx-auto py-6 px-4 md:px-6 lg:py-8",
                    {props.children}
                }
            }
        }
    }
}
