use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static NAV_MENU_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn generate_nav_id() -> usize {
    NAV_MENU_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, PartialEq)]
pub struct NavigationMenuContext {
    pub id: usize,
    pub open_item: Signal<Option<usize>>,
    pub set_open_item: Callback<Option<usize>>,
    pub viewport_ref: Signal<Option<MountedData>>,
    pub indicator_ref: Signal<Option<MountedData>>,
    pub orientation: NavigationMenuOrientation,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum NavigationMenuOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl NavigationMenuOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct NavigationMenuProps {
    #[props(default)]
    pub orientation: NavigationMenuOrientation,
    #[props(default = false)]
    pub skip_delay_show: bool,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NavigationMenu(props: NavigationMenuProps) -> Element {
    let mut open_item = use_signal(|| None);
    let viewport_ref = use_signal(|| None);
    let indicator_ref = use_signal(|| None);
    let id = use_hook(generate_nav_id);

    use_context_provider(|| NavigationMenuContext {
        id,
        open_item,
        set_open_item: Callback::new(move |new_val: Option<usize>| {
            open_item.set(new_val);
        }),
        viewport_ref,
        indicator_ref,
        orientation: props.orientation,
    });

    rsx! {
        nav {
            "data-slot": "navigation-menu",
            "data-orientation": props.orientation.as_str(),
            class: "{props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
