use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct HeroProps {
    #[props(default)]
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Hero(props: HeroProps) -> Element {
    rsx! {
        div { id: "hero", ..props.attributes, {props.children} }
    }
}
