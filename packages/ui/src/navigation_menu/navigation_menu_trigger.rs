use super::navigation_menu_item::NavigationMenuItemContext;
use crate::cn;
use crate::navigation_menu::NavigationMenuContext;
use dioxus::prelude::*;
use lucide_dioxus::ChevronDown;

const TRIGGER_BASE: &str = "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[state=open]:bg-accent/50";

#[derive(Clone, PartialEq, Props)]
pub struct NavigationMenuTriggerProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(default = false)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NavigationMenuTrigger(props: NavigationMenuTriggerProps) -> Element {
    let ctx = use_context::<NavigationMenuContext>();
    let item_ctx = use_context::<NavigationMenuItemContext>();
    let open_item = ctx.open_item;
    let set_open_item = ctx.set_open_item;

    // Read signal directly for bulletproof reactivity
    let is_open = open_item() == Some(item_ctx.id);

    let handle_click = move |_| {
        if props.disabled {
            return;
        }
        // Read open_item() directly inside the closure to get the live value
        let curr = open_item();
        let new_state = if curr == Some(item_ctx.id) {
            None
        } else {
            Some(item_ctx.id)
        };
        set_open_item.call(new_state);
    };

    let handle_mouse_enter = move |_| {
        if !props.disabled {
            set_open_item.call(Some(item_ctx.id));
        }
    };

    let chevron_class = if is_open {
        "relative top-[1px] ml-1 size-3 transition duration-300 group-data-[state=open]:rotate-180"
    } else {
        "relative top-[1px] ml-1 size-3 transition duration-200"
    };

    rsx! {
        button {
            r#type: "button",
            id: "{item_ctx.trigger_id}",
            "data-slot": "navigation-menu-trigger",
            "data-state": if is_open { "open" } else { "closed" },
            "aria-expanded": is_open,
            "aria-haspopup": "menu",
            "aria-controls": "{item_ctx.content_id}",
            disabled: props.disabled,
            class: cn(TRIGGER_BASE, &props.class),
            onclick: handle_click,
            onmouseenter: handle_mouse_enter,
            {props.children}
            ChevronDown { class: chevron_class }
        }
    }
}
