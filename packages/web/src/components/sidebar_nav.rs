use crate::docs::get_all_components;
use crate::Route;
use dioxus::prelude::*;
use ui::cn;

#[component]
pub fn SidebarNav(
    #[props(into, default)]
    active_slug: String,
    #[props(into, default)]
    class: String,
    /// Whether to use large text (for mobile popover)
    #[props(default = false)]
    large_text: bool,
) -> Element {
    let components = get_all_components();

    rsx! {
        nav {
            class: cn("flex flex-col gap-1", &class),
            "data-slot": "sidebar-nav",
            for component in components {
                SidebarLink {
                    slug: component.slug,
                    title: component.title,
                    is_active: component.slug == active_slug,
                    large_text,
                }
            }
        }
    }
}

#[component]
fn SidebarLink(slug: &'static str, title: &'static str, is_active: bool, large_text: bool) -> Element {
    let base_class = if large_text {
        "text-2xl font-medium py-1"
    } else {
        "text-sm py-1.5 px-2 rounded-md transition-colors"
    };

    let state_class = if is_active {
        "font-medium text-foreground bg-accent"
    } else {
        "text-muted-foreground hover:text-foreground hover:bg-accent"
    };

    rsx! {
        Link {
            to: Route::ComponentDoc { name: slug.to_string() },
            class: cn(base_class, state_class),
            "{title}"
        }
    }
}
