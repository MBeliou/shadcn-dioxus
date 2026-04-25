use crate::{InputGroupAddonAlign, InputGroupButtonSize};

#[test]
fn test_input_group_addon_align_as_str() {
    assert_eq!(InputGroupAddonAlign::InlineStart.as_str(), "inline-start");
    assert_eq!(InputGroupAddonAlign::InlineEnd.as_str(), "inline-end");
    assert_eq!(InputGroupAddonAlign::BlockStart.as_str(), "block-start");
    assert_eq!(InputGroupAddonAlign::BlockEnd.as_str(), "block-end");
}

#[test]
fn test_input_group_addon_align_class() {
    let start = InputGroupAddonAlign::InlineStart.class();
    assert!(start.contains("order-first"));
    assert!(start.contains("ps-3"));
    let end = InputGroupAddonAlign::InlineEnd.class();
    assert!(end.contains("order-last"));
    assert!(end.contains("pe-3"));
}

#[test]
fn test_input_group_button_size_class() {
    assert!(InputGroupButtonSize::Xs.class().contains("h-6"));
    assert!(InputGroupButtonSize::Sm.class().contains("h-8"));
    assert!(InputGroupButtonSize::IconXs.class().contains("size-6"));
    assert!(InputGroupButtonSize::IconSm.class().contains("size-8"));
}
