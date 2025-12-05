use dioxus::prelude::*;
use crate::cn;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarLoadingStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}
#[derive(Clone)]
pub struct AvatarCtx {
    pub state: Signal<AvatarLoadingStatus>,
}
#[derive(Clone, PartialEq, Props)]
pub struct AvatarProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = span)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let loading_state = use_signal(|| AvatarLoadingStatus::Idle);
    use_context_provider(|| AvatarCtx { state: loading_state });
    rsx! {
        span {
            class: cn("relative flex size-8 shrink-0 overflow-hidden rounded-full", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
