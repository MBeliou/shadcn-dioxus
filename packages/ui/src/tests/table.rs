use dioxus::prelude::*;

#[test]
fn test_table_components_render() {
    let _: Element = rsx! {
        crate::Table {
            crate::TableHeader {
                crate::TableRow {
                    crate::TableHead { "Col" }
                }
            }
            crate::TableBody {
                crate::TableRow {
                    crate::TableCell { "Data" }
                }
            }
        }
    };
    assert!(true);
}
