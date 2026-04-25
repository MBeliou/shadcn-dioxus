use crate::pagination::PaginationContext;
use crate::cn;
use dioxus::prelude::*;
use lucide_dioxus::ChevronLeft;

const BASE: &str = "h-9 w-9 inline-flex items-center justify-center rounded-md border border-input bg-background text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct PaginationPreviousProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn PaginationPrevious(props: PaginationPreviousProps) -> Element {
    let ctx = use_context::<PaginationContext>();
    let page = ctx.page;
    let set_page = ctx.set_page;
    let is_first = use_memo(move || page() <= 1);
    let classes = cn(BASE, &props.class);

    rsx! {
        li {
            button {
                r#type: "button",
                disabled: is_first(),
                class: "{classes}",
                "aria-label": "Go to previous page",
                onclick: move |_| {
                    if !is_first() {
                        set_page.call(page() - 1);
                    }
                },
                ChevronLeft { class: "h-4 w-4" }
                span { class: "sr-only", "Previous" }
            }
        }
    }
}
