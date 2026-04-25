use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static MENU_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn generate_menu_id() -> usize {
    MENU_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, PartialEq)]
pub struct MenubarMenuContext {
    pub id: usize,
}

#[derive(Clone, PartialEq, Props)]
pub struct MenubarMenuProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    let id = use_hook(generate_menu_id);
    use_context_provider(|| MenubarMenuContext { id });

    rsx! {
        div {
            "data-slot": "menubar-menu",
            class: "relative {props.class}",
            {props.children}
        }
    }
}
