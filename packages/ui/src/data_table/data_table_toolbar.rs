use crate::cn;
use crate::data_table::DataTableContext;
use crate::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};
use crate::input::Input;
use crate::button::{Button, ButtonVariant, ButtonSize};
use dioxus::prelude::*;
use lucide_dioxus::{Check, Search, X};

#[derive(Clone, PartialEq, Props)]
pub struct DataTableToolbarProps {
    #[props(default = true)]
    pub show_column_visibility: bool,
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn DataTableToolbar(props: DataTableToolbarProps) -> Element {
    let ctx = use_context::<DataTableContext>();
    let columns = ctx.columns;
    let mut filter_text = ctx.filter_text;
    let selected_rows = ctx.selected_rows;
    let total_filtered = ctx.total_filtered;
    let mut column_visibility = ctx.column_visibility;

    let has_selection = use_memo(move || !selected_rows().is_empty());
    let has_filter = use_memo(move || !filter_text.read().is_empty());

    let classes = cn(
        "flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between",
        &props.class,
    );

    rsx! {
        div { class: "{classes}",
            div { class: "flex flex-1 items-center gap-2",
                div { class: "relative flex-1 max-w-sm",
                    Search { class: "absolute left-2.5 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" }
                    Input {
                        class: "pl-8",
                        placeholder: "Filter rows...",
                        value: "{filter_text}",
                        oninput: move |e: FormEvent| filter_text.set(e.value()),
                    }
                    if has_filter() {
                        button {
                            class: "absolute right-2.5 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground hover:text-foreground",
                            onclick: move |_| filter_text.set(String::new()),
                            X { class: "h-4 w-4" }
                        }
                    }
                }
                if has_selection() {
                    span { class: "text-sm text-muted-foreground whitespace-nowrap",
                        "{selected_rows.len()} of {total_filtered()} row(s) selected"
                    }
                }
            }
            if props.show_column_visibility {
                DropdownMenu {
                    DropdownMenuTrigger {
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            "Columns"
                        }
                    }
                    DropdownMenuContent { side: "bottom".to_string(),
                        {
                            let cols = columns.read().clone();
                            let vis = column_visibility.read().clone();
                            rsx! {
                                for col in cols.iter() {
                                    {
                                        let col_id = col.id.clone();
                                        let col_header = col.header.clone();
                                        let is_visible = vis.contains(&col_id);
                                        rsx! {
                                            DropdownMenuItem {
                                                key: "{col_id}",
                                                onclick: {
                                                    let col_id = col_id.clone();
                                                    move |_| {
                                                        let mut new_vis = column_visibility.read().clone();
                                                        if new_vis.contains(&col_id) {
                                                            new_vis.retain(|x| x != &col_id);
                                                        } else {
                                                            // Clone again to avoid moving the captured value
                                                            new_vis.push(col_id.clone());
                                                        }
                                                        column_visibility.set(new_vis);
                                                    }
                                                },
                                                if is_visible {
                                                    Check { class: "mr-2 h-4 w-4" }
                                                }
                                                "{col_header}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
