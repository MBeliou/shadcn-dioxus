use crate::field::FieldLegendVariant;

#[test]
fn test_field_legend_variant_default() {
    assert_eq!(FieldLegendVariant::default(), FieldLegendVariant::Label);
}
