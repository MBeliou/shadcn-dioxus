use crate::{ToggleVariant, ToggleSize, toggle_variants};
use proptest::prelude::*;

proptest! {
    #[test]
    fn toggle_variant_size_combination_always_produces_classes(
        variant_idx in 0usize..2,
        size_idx in 0usize..3
    ) {
        let variants = [ToggleVariant::Default, ToggleVariant::Outline];
        let sizes = [ToggleSize::Default, ToggleSize::Sm, ToggleSize::Lg];
        let variant = variants[variant_idx];
        let size = sizes[size_idx];
        let classes = toggle_variants(variant, size);
        assert!(classes.contains("inline-flex"));
        assert!(!classes.is_empty());
    }
}
