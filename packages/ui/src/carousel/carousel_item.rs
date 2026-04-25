use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CarouselItemProps {
    pub index: usize,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    rsx! {
        div {
            "data-slot": "carousel-item",
            role: "group",
            "aria-roledescription": "slide",
            class: "shrink-0 {props.class}",
            style: "flex: 0 0 100%; width: 100%; height: 100%;",
            ..props.attributes,
            {props.children}
        }
    }
}
