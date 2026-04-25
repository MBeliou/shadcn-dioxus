use crate::cn;
use crate::navigation_menu::NavigationMenuContext;
use dioxus::prelude::*;

const LIST_BASE: &str = "group flex flex-1 list-none items-center justify-center space-x-1";

#[derive(Clone, PartialEq, Props)]
pub struct NavigationMenuListProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NavigationMenuList(props: NavigationMenuListProps) -> Element {
    let ctx = use_context::<NavigationMenuContext>();
    let orientation_class = match ctx.orientation {
        crate::navigation_menu::NavigationMenuOrientation::Horizontal => "flex-row",
        crate::navigation_menu::NavigationMenuOrientation::Vertical => "flex-col",
    };

    rsx! {
        ul {
            "data-slot": "navigation-menu-list",
            "data-orientation": ctx.orientation.as_str(),
            class: cn(&cn(LIST_BASE, orientation_class), &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
