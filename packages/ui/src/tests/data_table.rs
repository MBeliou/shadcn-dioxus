use crate::data_table::{DataTableColumn, SortDirection};

#[test]
fn test_data_table_column_new() {
    let col = DataTableColumn::new("name", "Name");
    assert_eq!(col.id, "name");
    assert_eq!(col.header, "Name");
    assert!(col.sortable);
}

#[test]
fn test_data_table_column_sortable_false() {
    let col = DataTableColumn::new("id", "ID").sortable(false);
    assert!(!col.sortable);
}

#[test]
fn test_sort_direction_default() {
    assert_eq!(SortDirection::default(), SortDirection::None);
}
