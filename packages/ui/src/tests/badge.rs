use crate::{BadgeVariant, badge_variants};

#[test]
fn test_badge_variant_default_class() {
    let c = BadgeVariant::Default.class();
    assert!(c.contains("bg-primary"));
    assert!(c.contains("text-primary-foreground"));
}

#[test]
fn test_badge_variant_secondary_class() {
    let c = BadgeVariant::Secondary.class();
    assert!(c.contains("bg-secondary"));
    assert!(c.contains("text-secondary-foreground"));
}

#[test]
fn test_badge_variant_destructive_class() {
    let c = BadgeVariant::Destructive.class();
    assert!(c.contains("bg-destructive"));
}

#[test]
fn test_badge_variant_outline_class() {
    let c = BadgeVariant::Outline.class();
    assert!(c.contains("text-foreground"));
    assert!(!c.contains("bg-"));
}

#[test]
fn test_badge_variant_as_str() {
    assert_eq!(BadgeVariant::Default.as_str(), "default");
    assert_eq!(BadgeVariant::Secondary.as_str(), "secondary");
    assert_eq!(BadgeVariant::Destructive.as_str(), "destructive");
    assert_eq!(BadgeVariant::Outline.as_str(), "outline");
}

#[test]
fn test_badge_variants_includes_base() {
    for v in [BadgeVariant::Default, BadgeVariant::Secondary, BadgeVariant::Destructive, BadgeVariant::Outline] {
        let result = badge_variants(v);
        assert!(result.contains("inline-flex"));
        assert!(result.contains("rounded-full"));
    }
}

#[test]
fn test_badge_variants_includes_variant_class() {
    let result = badge_variants(BadgeVariant::Destructive);
    assert!(result.contains("bg-destructive"));
}

#[test]
fn test_badge_variant_does_not_overlap() {
    let default = BadgeVariant::Default.class();
    let secondary = BadgeVariant::Secondary.class();
    assert_ne!(default, secondary);
}
