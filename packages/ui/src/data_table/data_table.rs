use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct DataTableColumn {
    pub id: String,
    pub header: String,
    pub sortable: bool,
}

impl DataTableColumn {
    pub fn new(id: impl Into<String>, header: impl Into<String>) -> Self {
        Self { id: id.into(), header: header.into(), sortable: true }
    }
    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SortDirection {
    #[default]
    None,
    Asc,
    Desc,
}

#[derive(Clone, PartialEq)]
pub struct DataTableContext {
    pub columns: Signal<Vec<DataTableColumn>>,
    pub rows: Signal<Vec<Vec<String>>>,
    pub sort_column: Signal<String>,
    pub sort_direction: Signal<SortDirection>,
    pub selected_rows: Signal<Vec<usize>>,
    pub column_visibility: Signal<Vec<String>>,
    pub filter_text: Signal<String>,
    pub page: Signal<usize>,
    pub page_size: Signal<usize>,
    pub visible_column_ids: Memo<Vec<String>>,
    pub processed_rows: Memo<Vec<(usize, Vec<String>)>>,
    pub total_filtered: Memo<usize>,
}

#[derive(Clone, PartialEq, Props)]
pub struct DataTableProps {
    pub columns: Vec<DataTableColumn>,
    pub rows: Vec<Vec<String>>,
    #[props(default = 10)]
    pub page_size: usize,
    #[props(default = true)]
    pub show_toolbar: bool,
    #[props(default = true)]
    pub show_pagination: bool,
    #[props(default)]
    pub on_selection_change: Option<Callback<Vec<usize>>>,
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DataTable(props: DataTableProps) -> Element {
    let columns = use_signal(|| props.columns.clone());
    let rows = use_signal(|| props.rows.clone());
    let sort_column = use_signal(String::new);
    let sort_direction = use_signal(|| SortDirection::None);
    let selected_rows = use_signal(Vec::<usize>::new);
    let column_visibility = use_signal(|| props.columns.iter().map(|c| c.id.clone()).collect());
    let filter_text = use_signal(String::new);
    let page = use_signal(|| 0);
    let page_size = use_signal(|| props.page_size);

    let visible_column_ids = use_memo(move || {
        let cols: Vec<DataTableColumn> = columns();
        let vis: Vec<String> = column_visibility();
        cols.iter().filter(|c| vis.contains(&c.id)).map(|c| c.id.clone()).collect::<Vec<_>>()
    });

    let processed_rows: Memo<Vec<(usize, Vec<String>)>> = use_memo(move || {
        let all_rows = rows();
        let mut result: Vec<(usize, Vec<String>)> = all_rows.into_iter().enumerate().collect();
        let filter = filter_text();
        if !filter.is_empty() {
            let lower = filter.to_lowercase();
            result.retain(|(_, row)| {
                row.iter().any(|cell| cell.to_lowercase().contains(&lower))
            });
        }
        let sort_col = sort_column();
        let sort_dir = sort_direction();
        if !sort_col.is_empty() && sort_dir != SortDirection::None {
            let vis_ids = visible_column_ids();
            if let Some(idx) = vis_ids.iter().position(|id| id == &sort_col) {
                result.sort_by(|a, b| {
                    let a_val = a.1.get(idx).map(|s| s.as_str()).unwrap_or("");
                    let b_val = b.1.get(idx).map(|s| s.as_str()).unwrap_or("");
                    match sort_dir {
                        SortDirection::Asc => a_val.cmp(b_val),
                        SortDirection::Desc => b_val.cmp(a_val),
                        SortDirection::None => std::cmp::Ordering::Equal,
                    }
                });
            }
        }
        result
    });

    let total_filtered: Memo<usize> = use_memo(move || processed_rows().len());

    use_context_provider(|| DataTableContext {
        columns: columns.clone(),
        rows: rows.clone(),
        sort_column: sort_column.clone(),
        sort_direction: sort_direction.clone(),
        selected_rows: selected_rows.clone(),
        column_visibility: column_visibility.clone(),
        filter_text: filter_text.clone(),
        page: page.clone(),
        page_size: page_size.clone(),
        visible_column_ids: visible_column_ids.clone(),
        processed_rows: processed_rows.clone(),
        total_filtered: total_filtered.clone(),
    });

    rsx! {
        div {
            "data-slot": "data-table",
            class: "flex flex-col gap-4 {props.class}",
            ..props.attributes,
            if props.show_toolbar {
                super::DataTableToolbar {}
            }
            div { class: "rounded-md border",
                crate::Table {
                    super::DataTableHeader {}
                    super::DataTableBody {}
                }
            }
            if props.show_pagination {
                super::DataTablePagination {}
            }
        }
    }
}
