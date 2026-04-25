use crate::carousel::{CarouselContext, CarouselOrientation};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CarouselIndicatorsProps {
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CarouselIndicators(props: CarouselIndicatorsProps) -> Element {
    let ctx = use_context::<CarouselContext>();
    let current_index = ctx.current_index;
    let total = ctx.total;
    let set_index = ctx.set_index;

    let position_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "bottom-2 left-1/2 -translate-x-1/2 flex",
        CarouselOrientation::Vertical => "right-2 top-1/2 -translate-y-1/2 flex-col",
    };

    let t = total();
    let curr = current_index();

    rsx! {
        div {
            "data-slot": "carousel-indicators",
            class: "absolute z-10 {position_class} gap-1 {props.class}",
            ..props.attributes,
            for i in 0..t {
                {
                    let is_active = i == curr;
                    let set_index = set_index.clone();
                    rsx! {
                        button {
                            key: "{i}",
                            r#type: "button",
                            role: "tab",
                            "aria-selected": is_active,
                            "aria-label": "Slide {i + 1}",
                            class: if is_active {
                                "inline-block h-2 w-2 rounded-full bg-primary"
                            } else {
                                "inline-block h-2 w-2 rounded-full bg-primary/30"
                            },
                            onclick: move |_| set_index.call(i),
                        }
                    }
                }
            }
        }
    }
}
