use crate::{ButtonVariant, ButtonSize, button_variants};

#[test]
fn test_button_variant_classes() {
    assert!(ButtonVariant::Default.class().contains("bg-primary"));
    assert!(ButtonVariant::Destructive.class().contains("bg-destructive"));
    assert!(ButtonVariant::Outline.class().contains("border"));
    assert!(ButtonVariant::Secondary.class().contains("bg-secondary"));
    assert!(ButtonVariant::Ghost.class().contains("hover:bg-accent"));
    assert!(ButtonVariant::Link.class().contains("underline"));
}

#[test]
fn test_button_size_classes() {
    assert!(ButtonSize::Default.class().contains("h-9"));
    assert!(ButtonSize::Sm.class().contains("h-8"));
    assert!(ButtonSize::Lg.class().contains("h-10"));
    assert!(ButtonSize::Icon.class().contains("size-9"));
    assert!(ButtonSize::IconSm.class().contains("size-8"));
    assert!(ButtonSize::IconLg.class().contains("size-10"));
}

#[test]
fn test_button_variants_includes_base() {
    for v in [ButtonVariant::Default, ButtonVariant::Outline, ButtonVariant::Ghost] {
        let result = button_variants(v, ButtonSize::Default);
        assert!(result.contains("inline-flex"));
    }
}

#[test]
fn test_button_variants_includes_size() {
    let result = button_variants(ButtonVariant::Default, ButtonSize::Lg);
    assert!(result.contains("h-10"));
}

#[test]
fn test_button_variants_includes_variant() {
    let result = button_variants(ButtonVariant::Destructive, ButtonSize::Default);
    assert!(result.contains("bg-destructive"));
}

#[test]
fn test_button_all_variants_distinct() {
    let variants = [ButtonVariant::Default, ButtonVariant::Destructive, ButtonVariant::Outline,
                    ButtonVariant::Secondary, ButtonVariant::Ghost, ButtonVariant::Link];
    for (i, v1) in variants.iter().enumerate() {
        for v2 in &variants[i+1..] {
            assert_ne!(v1.class(), v2.class());
        }
    }
}

#[test]
fn test_button_all_sizes_distinct() {
    let sizes = [ButtonSize::Default, ButtonSize::Sm, ButtonSize::Lg,
                 ButtonSize::Icon, ButtonSize::IconSm, ButtonSize::IconLg];
    for (i, s1) in sizes.iter().enumerate() {
        for s2 in &sizes[i+1..] {
            assert_ne!(s1.class(), s2.class());
        }
    }
}
