use crate::carousel::{CarouselContext, CarouselOrientation};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CarouselContentProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CarouselContent(props: CarouselContentProps) -> Element {
    let ctx = use_context::<CarouselContext>();
    let orientation = ctx.orientation;
    let current_index = ctx.current_index;

    let is_horizontal = matches!(orientation, CarouselOrientation::Horizontal);

    // Read signal to establish reactive dependency
    let current = current_index();

    let transform_style = if is_horizontal {
        format!("transform: translateX(-{}%)", current * 100)
    } else {
        format!("transform: translateY(-{}%)", current * 100)
    };

    let (wrapper_class, slider_class) = if is_horizontal {
        (
            "h-full".to_string(),
            "flex transition-transform duration-500 ease-in-out".to_string(),
        )
    } else {
        (
            "h-full relative".to_string(),
            "flex flex-col absolute inset-0 transition-transform duration-500 ease-in-out"
                .to_string(),
        )
    };

    rsx! {
        div {
            "data-slot": "carousel-content",
            class: "{wrapper_class} {props.class}",
            ..props.attributes,
            div {
                class: "{slider_class}",
                style: "{transform_style}",
                {props.children}
            }
        }
    }
}
