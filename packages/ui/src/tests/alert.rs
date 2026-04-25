use crate::AlertVariant;

#[test]
fn test_alert_variant_class() {
    let default = AlertVariant::Default.class();
    assert!(default.contains("bg-card"));
    assert!(default.contains("text-card-foreground"));
    let destructive = AlertVariant::Destructive.class();
    assert!(destructive.contains("text-destructive"));
    assert!(destructive.contains("bg-card"));
}
