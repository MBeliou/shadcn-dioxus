use dioxus::prelude::*;
use ui::ButtonVariant;
use views::{Blog, ButtonPage, ComponentView, Home};
mod components;
mod views;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[layout(components::Layout)]
    #[route("/docs/component")]
    ComponentView {},
    #[route("/docs/component/button")]
    ButtonPage {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
#[component]
fn WebNavbar() -> Element {
    rsx! {
        components::Navbar {
            Link {
                class: ui::button_variants(ButtonVariant::Ghost, ui::ButtonSize::Default),
                to: Route::Home {},
                "Home"
            }
            Link {
                class: ui::button_variants(ButtonVariant::Ghost, ui::ButtonSize::Default),
                to: Route::ComponentView {},
                "Components"
            }
            Link {
                class: ui::button_variants(ButtonVariant::Ghost, ui::ButtonSize::Default),
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }
        Outlet::<Route> {}
    }
}
