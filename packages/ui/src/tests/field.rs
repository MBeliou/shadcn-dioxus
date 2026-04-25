use crate::FieldLegendVariant;

#[test]
fn test_field_legend_default_is_label() {
    assert_eq!(FieldLegendVariant::default(), FieldLegendVariant::Label);
}
