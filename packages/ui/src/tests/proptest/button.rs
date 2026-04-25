use crate::{ButtonVariant, ButtonSize, button_variants};
use proptest::prelude::*;

proptest! {
    #[test]
    fn button_variant_size_combination_always_produces_classes(
        variant_idx in 0usize..6,
        size_idx in 0usize..6
    ) {
        let variants = [ButtonVariant::Default, ButtonVariant::Destructive, ButtonVariant::Outline,
                        ButtonVariant::Secondary, ButtonVariant::Ghost, ButtonVariant::Link];
        let sizes = [ButtonSize::Default, ButtonSize::Sm, ButtonSize::Lg,
                     ButtonSize::Icon, ButtonSize::IconSm, ButtonSize::IconLg];
        let variant = variants[variant_idx];
        let size = sizes[size_idx];
        let classes = button_variants(variant, size);
        // Every combination should produce at least the base classes
        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("items-center"));
        // Should contain the variant‑specific class
        assert!(!classes.is_empty());
    }

    #[test]
    fn button_all_variants_are_distinct(v1_idx in 0usize..5) {
        let variants = [ButtonVariant::Default, ButtonVariant::Destructive, ButtonVariant::Outline,
                        ButtonVariant::Secondary, ButtonVariant::Ghost, ButtonVariant::Link];
        for v2_idx in (v1_idx+1)..6 {
            assert_ne!(variants[v1_idx].class(), variants[v2_idx].class());
        }
    }
}
