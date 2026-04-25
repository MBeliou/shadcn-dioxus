use crate::{EmptyMediaVariant, empty_media_variants};

#[test]
fn test_empty_media_variant_class() {
    assert!(EmptyMediaVariant::Default.class().contains("bg-transparent"));
    assert!(EmptyMediaVariant::Icon.class().contains("bg-muted"));
    assert!(EmptyMediaVariant::Icon.class().contains("rounded-lg"));
}

#[test]
fn test_empty_media_variant_as_str() {
    assert_eq!(EmptyMediaVariant::Default.as_str(), "default");
    assert_eq!(EmptyMediaVariant::Icon.as_str(), "icon");
}

#[test]
fn test_empty_media_variants_includes_base() {
    let result = empty_media_variants(EmptyMediaVariant::Icon);
    assert!(result.contains("flex"));
    assert!(result.contains("shrink-0"));
}
