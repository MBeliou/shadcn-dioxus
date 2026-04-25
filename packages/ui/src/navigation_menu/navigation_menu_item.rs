use crate::navigation_menu::NavigationMenuContext;
use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static ITEM_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn generate_item_id() -> usize {
    ITEM_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, PartialEq)]
pub struct NavigationMenuItemContext {
    pub id: usize,
    pub content_id: String,
    pub trigger_id: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct NavigationMenuItemProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NavigationMenuItem(props: NavigationMenuItemProps) -> Element {
    let ctx = use_context::<NavigationMenuContext>();
    let set_open_item = ctx.set_open_item;

    let id = use_hook(generate_item_id);
    let content_id = use_hook(|| format!("nav-menu-content-{}", id));
    let trigger_id = use_hook(|| format!("nav-menu-trigger-{}", id));

    use_context_provider(|| NavigationMenuItemContext {
        id,
        content_id: content_id.clone(),
        trigger_id: trigger_id.clone(),
    });

    rsx! {
        li {
            "data-slot": "navigation-menu-item",
            class: "relative {props.class}",
            onmouseleave: move |_| set_open_item.call(None),
            ..props.attributes,
            {props.children}
        }
    }
}
