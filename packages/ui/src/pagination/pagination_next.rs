use crate::pagination::PaginationContext;
use crate::cn;
use dioxus::prelude::*;
use lucide_dioxus::ChevronRight;

const BASE: &str = "h-9 w-9 inline-flex items-center justify-center rounded-md border border-input bg-background text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct PaginationNextProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn PaginationNext(props: PaginationNextProps) -> Element {
    let ctx = use_context::<PaginationContext>();
    let page = ctx.page;
    let total = ctx.total_pages;
    let set_page = ctx.set_page;
    let is_last = use_memo(move || page() >= total());
    let classes = cn(BASE, &props.class);

    rsx! {
        li {
            button {
                r#type: "button",
                disabled: is_last(),
                class: "{classes}",
                "aria-label": "Go to next page",
                onclick: move |_| {
                    if !is_last() {
                        set_page.call(page() + 1);
                    }
                },
                span { class: "sr-only", "Next" }
                ChevronRight { class: "h-4 w-4" }
            }
        }
    }
}
