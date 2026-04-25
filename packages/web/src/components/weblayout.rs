use dioxus::prelude::*;
use crate::components::{FullLayout, SidebarLayout, Navbar};
use crate::Route;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css", CssAssetOptions::new());

#[component]
pub fn WebNavbar() -> Element {
    let route = use_route::<Route>();

    let is_docs = matches!(&route,
        Route::ComponentDoc { .. } |
        Route::ComponentView {} |
        Route::InstallationView {} |
        Route::ThemingView {}
    );

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "min-h-screen flex flex-col",
            Navbar {
                Link {
                    class: ui::button_variants(ui::ButtonVariant::Ghost, ui::ButtonSize::Default),
                    to: Route::Home {},
                    "Home"
                }
                Link {
                    class: ui::button_variants(ui::ButtonVariant::Ghost, ui::ButtonSize::Default),
                    to: Route::ComponentView {},
                    "Components"
                }
            }
            div { class: "grow",
                if is_docs {
                    SidebarLayout {
                        Outlet::<Route> {}
                    }
                } else {
                    FullLayout {
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let path = route.join("/");
    rsx! {
        div { class: "container mx-auto flex-1 py-12 text-center",
            h1 { class: "text-4xl font-bold mb-4", "404" }
            p { class: "text-muted-foreground mb-6", "The page /{path} does not exist." }
            Link {
                class: ui::button_variants(ui::ButtonVariant::Default, ui::ButtonSize::Default),
                to: Route::Home {},
                "Go Home"
            }
        }
    }
}
