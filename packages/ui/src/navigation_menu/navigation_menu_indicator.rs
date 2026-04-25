use crate::cn;
use crate::navigation_menu::NavigationMenuContext;
use dioxus::prelude::*;

const INDICATOR_BASE: &str = "data-[state=visible]:animate-in data-[state=hidden]:animate-out data-[state=hidden]:fade-out data-[state=visible]:fade-in data-[state=hidden]:zoom-out-100 data-[state=visible]:zoom-in-100 top-full z-[1] flex h-1.5 items-end justify-center overflow-hidden data-[state=hidden]:opacity-0";

const ARROW_BASE: &str = "relative top-[60%] size-2 rotate-45 rounded-tl-sm bg-border shadow-md";

#[derive(Clone, PartialEq, Props)]
pub struct NavigationMenuIndicatorProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub force_mount: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NavigationMenuIndicator(props: NavigationMenuIndicatorProps) -> Element {
    let ctx = use_context::<NavigationMenuContext>();
    let open_item = ctx.open_item;

    // Wrap in a closure so the signal is read INSIDE the rsx! macro
    let is_visible = move || open_item().is_some();

    // Calculate outside rsx! so it updates reactively but doesn't break syntax
    let data_state = move || {
        if open_item().is_some() {
            "visible"
        } else {
            "hidden"
        }
    };

    rsx! {
        // Conditional rendering MUST be inside rsx! for Dioxus to track the dependency
        if props.force_mount || is_visible() {
            div {
                "data-slot": "navigation-menu-indicator",
                "data-state": data_state(),
                "aria-hidden": "true",
                class: cn(INDICATOR_BASE, &props.class),
                ..props.attributes,
                div {
                    class: ARROW_BASE,
                }
            }
        }
    }
}
