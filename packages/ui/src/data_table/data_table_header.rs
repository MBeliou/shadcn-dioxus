use crate::data_table::{DataTableContext, SortDirection};
use dioxus::prelude::*;
use lucide_dioxus::ChevronDown;

#[component]
pub fn DataTableHeader() -> Element {
    let ctx = use_context::<DataTableContext>();
    let visible_column_ids = ctx.visible_column_ids;
    let mut sort_column = ctx.sort_column;
    let mut sort_direction = ctx.sort_direction;

    let cols = visible_column_ids.read().clone();
    let cur_sort = sort_column.read().clone();
    let cur_dir = sort_direction.read().clone();

    rsx! {
        crate::table::TableHeader {
            crate::table::TableRow {
                class: "border-b hover:bg-muted/50",
                crate::table::TableHead { class: "w-[40px]", "" }
                for col_id in &cols {
                    {
                        let id = col_id.clone();
                        let is_sorted = cur_sort == *col_id;
                        let dir = if is_sorted { cur_dir } else { SortDirection::None };
                        let all_cols = ctx.columns.read().clone();
                        let header_text = all_cols
                            .iter()
                            .find(|c| c.id == *col_id)
                            .map(|c| c.header.clone())
                            .unwrap_or_default();
                        let is_sortable = all_cols
                            .iter()
                            .find(|c| c.id == *col_id)
                            .map(|c| c.sortable)
                            .unwrap_or(false);
                        rsx! {
                            crate::table::TableHead {
                                key: "{id}",
                                class: if is_sortable {
                                    "h-9 cursor-pointer select-none"
                                } else {
                                    "h-9"
                                },
                                button {
                                    class: "flex items-center gap-1 w-full h-full bg-transparent border-none text-left font-medium text-muted-foreground hover:text-foreground",
                                    onclick: {
                                        let id_owned = id.clone();
                                        move |_| {
                                            let current = sort_column.read().clone();
                                            let current_dir = sort_direction.read().clone();
                                            if current == id_owned {
                                                match current_dir {
                                                    SortDirection::Asc => sort_direction.set(SortDirection::Desc),
                                                    SortDirection::Desc => {
                                                        sort_column.set(String::new());
                                                        sort_direction.set(SortDirection::None);
                                                    }
                                                    SortDirection::None => sort_direction.set(SortDirection::Asc),
                                                }
                                            } else {
                                                // Clone the string to avoid moving it out of the closure environment
                                                sort_column.set(id_owned.clone());
                                                sort_direction.set(SortDirection::Asc);
                                            }
                                        }
                                    },
                                    "{header_text}"
                                    match dir {
                                        SortDirection::Asc => rsx! {
                                            ChevronDown { class: "ml-1 h-4 w-4 text-foreground/70 -rotate-90" }
                                        },
                                        SortDirection::Desc => rsx! {
                                            ChevronDown { class: "ml-1 h-4 w-4 text-foreground/70" }
                                        },
                                        SortDirection::None => rsx! {},
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
