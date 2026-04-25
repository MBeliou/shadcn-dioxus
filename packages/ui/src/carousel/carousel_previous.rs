use crate::carousel::{CarouselContext, CarouselOrientation};
use crate::cn;
use dioxus::prelude::*;
use lucide_dioxus::ChevronLeft;

const BUTTON_BASE: &str = "absolute z-10 inline-flex h-8 w-8 items-center justify-center rounded-full border border-input bg-background/80 shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct CarouselPreviousProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CarouselPrevious(props: CarouselPreviousProps) -> Element {
    let ctx = use_context::<CarouselContext>();
    let current_index = ctx.current_index;
    let total = ctx.total;
    let set_index = ctx.set_index;

    let position = match ctx.orientation {
        CarouselOrientation::Horizontal => "left-2 top-1/2 -translate-y-1/2",
        CarouselOrientation::Vertical => "left-1/2 top-2 -translate-x-1/2 rotate-90",
    };

    let classes = cn(&format!("{} {}", BUTTON_BASE, position), &props.class);
    let is_disabled = props.disabled || total() == 0 || current_index() == 0;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "carousel-previous",
            disabled: is_disabled,
            class: "{classes}",
            onclick: move |_| {
                let t = total();
                let curr = current_index();
                if t > 0 && curr > 0 {
                    set_index.call(curr - 1);
                }
            },
            ChevronLeft { class: "h-4 w-4" }
            span { class: "sr-only", "Previous slide" }
        }
    }
}
