use crate::ToggleGroupType;

#[test]
fn test_toggle_group_type_as_str() {
    assert_eq!(ToggleGroupType::Single.as_str(), "single");
    assert_eq!(ToggleGroupType::Multiple.as_str(), "multiple");
}
