use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SkeletonProps {
    #[props(into, default)]
    pub class: String,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    rsx! {
        div {
            "data-slot": "skeleton",
            class: cn("bg-accent animate-pulse rounded-md", &props.class),
            ..props.attributes,
        }
    }
}
