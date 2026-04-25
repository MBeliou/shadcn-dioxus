use crate::Direction;

#[test]
fn test_direction_as_str() {
    assert_eq!(Direction::Horizontal.as_str(), "horizontal");
    assert_eq!(Direction::Vertical.as_str(), "vertical");
}

#[test]
fn test_direction_flex_dir() {
    assert!(Direction::Horizontal.flex_dir().contains("flex-row"));
    assert!(Direction::Vertical.flex_dir().contains("flex-col"));
}
