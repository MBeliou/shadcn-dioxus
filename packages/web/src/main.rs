use dioxus::prelude::*;
use ui::ButtonVariant;
use views::{ComponentDoc, ComponentView, Home};
mod components;
mod docs;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[layout(components::Layout)]
    #[route("/docs/components")]
    ComponentView {},
    #[route("/docs/components/:name")]
    ComponentDoc { name: String },
}
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!(
    "/assets/tailwind.css",
    CssAssetOptions::new().with_preload(true)
);
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
        }
        Outlet::<Route> {}
    }
}
