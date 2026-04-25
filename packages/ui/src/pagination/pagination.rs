use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PaginationContext {
    pub page: Signal<usize>,
    pub set_page: Callback<usize>,
    pub total_pages: Signal<usize>,
}

#[derive(Clone, PartialEq, Props)]
pub struct PaginationProps {
    #[props(default = 1)]
    pub page: usize,
    #[props(default = 1)]
    pub total_pages: usize,
    #[props(default)]
    pub on_change: Option<Callback<usize>>,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let mut page = use_signal(|| props.page);
    let total_pages = use_signal(|| props.total_pages);

    let set_page = use_callback(move |val: usize| {
        page.set(val);
        if let Some(cb) = &props.on_change {
            cb.call(val);
        }
    });

    use_context_provider(|| PaginationContext { page, set_page, total_pages });

    rsx! {
        nav {
            "data-slot": "pagination",
            role: "navigation",
            "aria-label": "pagination",
            class: "mx-auto flex w-full justify-center {props.class}",
            {props.children}
        }
    }
}
