use crate::docs::get_all_components;
use crate::Route;
use dioxus::prelude::*;
use ui::{button_variants, cn, ButtonSize, ButtonVariant};

#[component]
pub fn SidebarNav(
    #[props(into, default)]
    active_slug: String,
    #[props(into, default)]
    class: String,
    #[props(default = false)]
    large_text: bool,
    /// Callback to close the mobile sidebar
    #[props(default)]
    on_close: Option<Callback<()>>,
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
                    is_new: component.new,
                    large_text,
                    on_close: on_close.clone(),
                }
            }
        }
    }
}

#[component]
fn SidebarLink(
    slug: &'static str,
    title: &'static str,
    is_active: bool,
    is_new: bool,
    large_text: bool,
    #[props(default)]
    on_close: Option<Callback<()>>,
) -> Element {
    let _base_class = if large_text {
        "text-2xl font-medium py-1"
    } else {
        "text-sm py-1.5 px-2 rounded-md transition-colors"
    };
    let state_class = if is_active {
        "font-medium text-foreground bg-accent text-start justify-start"
    } else {
        "text-muted-foreground hover:text-foreground hover:bg-accent text-start justify-start"
    };
    rsx! {
        Link {
            to: Route::ComponentDoc {
                name: slug.to_string(),
            },
            class: cn(&button_variants(ButtonVariant::Ghost, ButtonSize::Sm), state_class),
            onclick: move |_| {
                if let Some(cb) = &on_close {
                    cb.call(());
                }
            },
            "{title}"
            if is_new {
                span { class: "ml-2 inline-flex items-center rounded-full bg-primary px-2 py-0.5 text-xs font-medium text-primary-foreground", "New" }
            }
        }
    }
}

#[component]
pub fn SidebarLinkv2(
    to: NavigationTarget,
    is_active: bool,
    #[props(default = false)]
    large_text: bool,
    children: Element,
    /// Callback to close the mobile sidebar
    #[props(default)]
    on_close: Option<Callback<()>>,
) -> Element {
    let _base_class = if large_text {
        "text-2xl font-medium py-1"
    } else {
        "text-sm py-1.5 px-2 rounded-md transition-colors"
    };
    let state_class = if is_active {
        "font-medium text-foreground bg-accent text-start justify-start"
    } else {
        "text-muted-foreground hover:text-foreground hover:bg-accent text-start justify-start"
    };
    rsx! {
        Link {
            to: to,
            class: cn(&button_variants(ButtonVariant::Ghost, ButtonSize::Sm), state_class),
            onclick: move |_| {
                if let Some(cb) = &on_close {
                    cb.call(());
                }
            },
            {children}
        }
    }
}
