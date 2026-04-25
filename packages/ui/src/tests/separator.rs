use crate::separator::SeparatorOrientation;

#[test]
fn test_separator_orientation_as_str() {
    assert_eq!(SeparatorOrientation::Vertical.as_str(), "vertical");
    assert_eq!(SeparatorOrientation::Horizontal.as_str(), "horizontal");
}

#[test]
fn test_separator_orientation_base_class() {
    assert!(SeparatorOrientation::Vertical.base_class().contains("w-px"));
    assert!(SeparatorOrientation::Horizontal.base_class().contains("h-px"));
}
