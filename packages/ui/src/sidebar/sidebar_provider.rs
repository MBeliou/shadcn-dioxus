use dioxus::prelude::*;
use super::sidebar_context::{SidebarContext, SidebarVariant};

#[derive(Clone, PartialEq, Props)]
pub struct SidebarProviderProps {
    #[props(default)]
    pub default_open: bool,
    #[props(default)]
    pub variant: SidebarVariant,
    pub children: Element,
}

#[component]
pub fn SidebarProvider(props: SidebarProviderProps) -> Element {
    let mut open = use_signal(|| props.default_open);
    let mut open_mobile = use_signal(|| false);

    let set_open = use_callback(move |val: bool| open.set(val));
    let set_open_mobile = use_callback(move |val: bool| open_mobile.set(val));

    use_context_provider(|| SidebarContext {
        open,
        set_open,
        open_mobile,
        set_open_mobile,
        variant: props.variant,
    });

    rsx! {
        {props.children}
    }
}
