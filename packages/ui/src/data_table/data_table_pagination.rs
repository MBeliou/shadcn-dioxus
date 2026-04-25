use crate::cn;
use crate::data_table::DataTableContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DataTablePaginationProps {
    #[props(default = vec![10, 20, 30, 40, 50])]
    pub page_size_options: Vec<usize>,
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn DataTablePagination(props: DataTablePaginationProps) -> Element {
    let ctx = use_context::<DataTableContext>();
    let mut page = ctx.page;
    let mut page_size = ctx.page_size;
    let total_filtered = ctx.total_filtered;

    let total_pages = use_memo(move || {
        let ps = page_size.read().clone();
        let total = total_filtered();
        if ps > 0 && total > 0 {
            (total + ps - 1) / ps
        } else {
            1
        }
    });

    let current_page = *page.read();   // deref to usize (Copy)
    let ps = *page_size.read();
    let total = total_filtered();
    let start = if total > 0 { current_page * ps + 1 } else { 0 };
    let end = std::cmp::min((current_page + 1) * ps, total);

    let can_prev = current_page > 0;
    let can_next = current_page < total_pages() - 1;

    let classes = cn(
        "flex flex-col-reverse gap-3 sm:flex-row sm:items-center sm:justify-between px-2",
        &props.class,
    );

    rsx! {
        div { class: "{classes}",
            div { class: "text-sm text-muted-foreground",
                if total > 0 {
                    "Showing {start} to {end} of {total} entries"
                } else {
                    "0 entries"
                }
            }
            div { class: "flex items-center gap-6 lg:gap-8",
                div { class: "flex items-center gap-2",
                    label { class: "text-sm font-medium", "Rows per page" }
                    select {
                        class: "border-input shadow-xs h-8 w-[70px] rounded-md border bg-transparent px-1 text-sm outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
                        oninput: move |e: FormEvent| {
                            if let Ok(val) = e.value().parse::<usize>() {
                                page_size.set(val);
                                page.set(0);
                            }
                        },
                        for size in &props.page_size_options {
                            option {
                                value: "{size}",
                                selected: *size == ps,
                                "{size}"
                            }
                        }
                    }
                }
                div { class: "flex items-center gap-1",
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 w-8 items-center justify-center rounded-md border border-input bg-transparent text-sm font-medium disabled:pointer-events-none disabled:opacity-50",
                        disabled: !can_prev,
                        // avoid overlapping borrows by reading into a local
                        onclick: {
                            let mut page = page.clone();
                            move |_| {
                                let current = *page.read();
                                page.set(current - 1);
                            }
                        },
                        "‹"
                    }
                    span { class: "text-sm font-medium",
                        "Page {current_page} of {total_pages}"
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 w-8 items-center justify-center rounded-md border border-input bg-transparent text-sm font-medium disabled:pointer-events-none disabled:opacity-50",
                        disabled: !can_next,
                        onclick: {
                            let mut page = page.clone();
                            move |_| {
                                let current = *page.read();
                                page.set(current + 1);
                            }
                        },
                        "›"
                    }
                }
            }
        }
    }
}
