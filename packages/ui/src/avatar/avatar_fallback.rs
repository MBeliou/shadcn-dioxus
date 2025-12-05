use dioxus::prelude::*;
use futures_timer::Delay;
use std::time::Duration;
use crate::{cn, AvatarCtx, AvatarLoadingStatus};
#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    #[props(default)]
    pub delay_ms: Option<u64>,
    pub children: Element,
    #[props(extends = span)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let avatarCtx = use_context::<AvatarCtx>();
    let mut can_render = use_signal(|| props.delay_ms.is_none());
    use_effect(move || {
        if let Some(delay) = props.delay_ms {
            can_render.set(false);
            spawn(async move {
                Delay::new(Duration::from_millis(delay)).await;
                can_render.set(true);
            });
        } else {
            can_render.set(true);
        }
    });
    rsx! {
        if can_render() && (avatarCtx.state)() != AvatarLoadingStatus::Loaded {
            span { ..props.attributes,{props.children} }
        }
    }
}
