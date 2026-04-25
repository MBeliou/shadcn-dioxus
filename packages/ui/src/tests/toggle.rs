use crate::{ToggleVariant, ToggleSize, toggle_variants};

#[test]
fn test_toggle_variant_class() {
    assert!(ToggleVariant::Default.class().contains("bg-transparent"));
    assert!(ToggleVariant::Outline.class().contains("border"));
}

#[test]
fn test_toggle_size_class() {
    assert!(ToggleSize::Default.class().contains("h-9"));
    assert!(ToggleSize::Sm.class().contains("h-8"));
    assert!(ToggleSize::Lg.class().contains("h-10"));
}

#[test]
fn test_toggle_variants_includes_base() {
    let result = toggle_variants(ToggleVariant::Default, ToggleSize::Default);
    assert!(result.contains("inline-flex"));
}

#[test]
fn test_toggle_all_variants_distinct() {
    assert_ne!(ToggleVariant::Default.class(), ToggleVariant::Outline.class());
}

#[test]
fn test_toggle_all_sizes_distinct() {
    assert_ne!(ToggleSize::Default.class(), ToggleSize::Sm.class());
    assert_ne!(ToggleSize::Sm.class(), ToggleSize::Lg.class());
}
