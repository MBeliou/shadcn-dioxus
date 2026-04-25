use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct NavbarProps {
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        nav { id: "navbar", ..props.attributes, {props.children} }
    }
}
