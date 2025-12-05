use dioxus::prelude::*;
use crate::{cn, AvatarCtx, AvatarLoadingStatus};
#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {
    pub src: String,
    #[props(default)]
    pub alt: String,
    #[props(extends = img)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let mut avatarCtx = use_context::<AvatarCtx>();
    let handle_load = move |_| {
        avatarCtx.state.set(AvatarLoadingStatus::Loaded);
    };
    let handle_error = move |_| {
        avatarCtx.state.set(AvatarLoadingStatus::Error);
    };
    rsx! {
        img {
            src: props.src,
            alt: props.alt,
            onload: handle_load,
            onerror: handle_error,
            ..props.attributes,
        }
    }
}
