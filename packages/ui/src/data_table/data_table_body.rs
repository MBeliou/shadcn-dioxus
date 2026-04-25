use crate::data_table::DataTableContext;
use dioxus::prelude::*;
use lucide_dioxus::Check;

#[component]
pub fn DataTableBody() -> Element {
    let ctx = use_context::<DataTableContext>();
    let visible_column_ids = ctx.visible_column_ids;
    let mut selected_rows = ctx.selected_rows;
    let columns = ctx.columns;

    // Reactively read the memo to decide emptiness
    if (ctx.total_filtered)() == 0 {
        return rsx! {
            crate::table::TableBody {
                crate::table::TableRow {
                    crate::table::TableCell {
                        column_span: (visible_column_ids().len() + 1) as u32,
                        class: "h-24 text-center",
                        div { class: "text-muted-foreground text-center", "No results." }
                    }
                }
            }
        };
    }

    let rows = (ctx.processed_rows)();

    rsx! {
        crate::table::TableBody {
            for (original_idx, row_data) in rows.iter() {
                {
                    let idx = *original_idx;
                    let row_clone = row_data.clone();
                    let is_selected = use_memo(move || selected_rows().contains(&idx));
                    let cols = visible_column_ids();
                    let all_cols = columns.read().clone();
                    rsx! {
                        crate::table::TableRow {
                            key: "{idx}",
                            class: "data-[state=selected]:bg-muted/50",
                            crate::table::TableCell { class: "w-[40px]",
                                button {
                                    r#type: "button",
                                    class: "flex h-4 w-4 items-center justify-center rounded border border-primary data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground",
                                    "data-state": if is_selected() { "checked" } else { "unchecked" },
                                    "aria-checked": is_selected(),
                                    onclick: move |_| {
                                        let mut sel = selected_rows();
                                        if sel.contains(&idx) {
                                            sel.retain(|x| x != &idx);
                                        } else {
                                            sel.push(idx);
                                        }
                                        selected_rows.set(sel);
                                    },
                                    if is_selected() {
                                        Check { class: "h-3.5 w-3.5" }
                                    }
                                }
                            }
                            for col_id in &cols {
                                {
                                    let cell_value = all_cols
                                        .iter()
                                        .position(|c| &c.id == col_id)
                                        .and_then(|pos| row_clone.get(pos).cloned())
                                        .unwrap_or_default();
                                    rsx! {
                                        crate::table::TableCell { key: "{col_id}", "{cell_value}" }
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
