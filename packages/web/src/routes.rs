use dioxus::prelude::*;
use crate::components::{WebNavbar, NotFound};
use crate::views::{
    ComponentDoc, ComponentView, Home, InstallationView, PlaygroundExample, ThemingView,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/examples/playground")]
    PlaygroundExample {},
    #[route("/docs/installation")]
    InstallationView {},
    #[route("/docs/theming")]
    ThemingView {},
    #[route("/docs/components")]
    ComponentView {},
    #[route("/docs/components/:name")]
    ComponentDoc { name: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
