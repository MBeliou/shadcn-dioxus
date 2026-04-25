use crate::{NavigationMenuOrientation, navigation_menu_trigger_style};

#[test]
fn test_navigation_menu_orientation_as_str() {
    assert_eq!(NavigationMenuOrientation::Horizontal.as_str(), "horizontal");
    assert_eq!(NavigationMenuOrientation::Vertical.as_str(), "vertical");
}

#[test]
fn test_navigation_menu_trigger_style() {
    let style = navigation_menu_trigger_style();
    assert!(style.contains("inline-flex"));
    assert!(style.contains("rounded-md"));
    assert!(style.contains("bg-background"));
}

#[test]
fn test_orientation_distinct() {
    assert_ne!(NavigationMenuOrientation::Horizontal.as_str(), NavigationMenuOrientation::Vertical.as_str());
}
