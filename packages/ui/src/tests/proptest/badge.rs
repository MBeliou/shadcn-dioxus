use crate::{BadgeVariant, badge_variants};
use proptest::prelude::*;

proptest! {
    #[test]
    fn badge_variant_always_has_classes(variant_idx in 0usize..4) {
        let variants = [BadgeVariant::Default, BadgeVariant::Secondary, BadgeVariant::Destructive, BadgeVariant::Outline];
        let variant = variants[variant_idx];
        let classes = badge_variants(variant);
        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("rounded-full"));
        assert!(!classes.is_empty());
    }

    #[test]
    fn badge_all_variants_are_distinct(v1_idx in 0usize..3) {
        let variants = [BadgeVariant::Default, BadgeVariant::Secondary, BadgeVariant::Destructive, BadgeVariant::Outline];
        for v2_idx in (v1_idx+1)..4 {
            assert_ne!(variants[v1_idx].class(), variants[v2_idx].class());
        }
    }
}
