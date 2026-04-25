use crate::{ItemVariant, ItemSize, item_variants};

#[test]
fn test_item_variant_class() {
    assert!(ItemVariant::Default.class().contains("bg-transparent"));
    assert!(ItemVariant::Outline.class().contains("border-border"));
    assert!(ItemVariant::Muted.class().contains("bg-muted/50"));
}

#[test]
fn test_item_variant_as_str() {
    assert_eq!(ItemVariant::Default.as_str(), "default");
    assert_eq!(ItemVariant::Outline.as_str(), "outline");
    assert_eq!(ItemVariant::Muted.as_str(), "muted");
}

#[test]
fn test_item_size_class() {
    assert!(ItemSize::Default.class().contains("gap-4"));
    assert!(ItemSize::Sm.class().contains("gap-2.5"));
}

#[test]
fn test_item_size_as_str() {
    assert_eq!(ItemSize::Default.as_str(), "default");
    assert_eq!(ItemSize::Sm.as_str(), "sm");
}

#[test]
fn test_item_variants_includes_base() {
    let result = item_variants(ItemVariant::Default, ItemSize::Default);
    assert!(result.contains("inline-flex"));
    assert!(result.contains("rounded-md"));
}
