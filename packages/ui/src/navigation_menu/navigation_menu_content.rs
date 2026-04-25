use super::navigation_menu_item::NavigationMenuItemContext;
use crate::cn;
use crate::navigation_menu::NavigationMenuContext;
use dioxus::prelude::*;

const CONTENT_BASE: &str = "absolute left-0 top-full z-50";
const INNER_BASE: &str = "bg-popover text-popover-foreground relative z-50 w-64 overflow-hidden rounded-md border p-1 shadow-lg md:w-auto";

#[derive(Clone, PartialEq, Props)]
pub struct NavigationMenuContentProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub force_mount: bool,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NavigationMenuContent(props: NavigationMenuContentProps) -> Element {
    let ctx = use_context::<NavigationMenuContext>();
    let item_ctx = use_context::<NavigationMenuItemContext>();
    let open_item = ctx.open_item;

    // Wrap in a closure so the signal is read INSIDE the rsx! macro
    let is_open = move || open_item() == Some(item_ctx.id);

    rsx! {
        // Conditional rendering MUST be inside rsx! for Dioxus to track the dependency
        if props.force_mount || is_open() {
            div {
                "data-slot": "navigation-menu-content",
                "data-state": "open",
                id: "{item_ctx.content_id}",
                "aria-labelledby": "{item_ctx.trigger_id}",
                role: "menu",
                class: cn(CONTENT_BASE, &props.class),
                ..props.attributes,
                div {
                    class: INNER_BASE,
                    {props.children}
                }
            }
        }
    }
}
