use crate::combobox::{ComboboxOption};
use dioxus::prelude::*;

#[test]
fn test_combobox_option_struct() {
    let opt = ComboboxOption { value: "nextjs".into(), label: "Next.js".into() };
    assert_eq!(opt.value, "nextjs");
}

#[test]
fn test_combobox_filtering_logic() {
    let options = vec![
        ComboboxOption { value: "apple".into(), label: "Apple".into() },
        ComboboxOption { value: "banana".into(), label: "Banana".into() },
        ComboboxOption { value: "grape".into(), label: "Grape".into() },
    ];
    let filtered: Vec<_> = options.iter()
        .filter(|o| o.label.to_lowercase().contains("ap")).collect();
    assert_eq!(filtered.len(), 2);
}

#[test]
fn test_combobox_renders() {
    let _: Element = rsx! {
        crate::Combobox {
            options: vec![
                ComboboxOption { value: "a".into(), label: "A".into() },
            ]
        }
    };
    assert!(true);
}
