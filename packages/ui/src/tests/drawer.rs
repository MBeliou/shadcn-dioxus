use crate::DrawerSide;

#[test]
fn test_drawer_side_as_str() {
    assert_eq!(DrawerSide::Right.as_str(), "right");
    assert_eq!(DrawerSide::Left.as_str(), "left");
    assert_eq!(DrawerSide::Top.as_str(), "top");
    assert_eq!(DrawerSide::Bottom.as_str(), "bottom");
}

#[test]
fn test_drawer_side_classes() {
    let right = DrawerSide::Right.classes();
    assert!(right.contains("inset-y-0"));
    assert!(right.contains("right-0"));
    let left = DrawerSide::Left.classes();
    assert!(left.contains("left-0"));
    let top = DrawerSide::Top.classes();
    assert!(top.contains("top-0"));
    let bottom = DrawerSide::Bottom.classes();
    assert!(bottom.contains("bottom-0"));
}

#[test]
fn test_drawer_side_all_distinct() {
    let sides = [DrawerSide::Right, DrawerSide::Left, DrawerSide::Top, DrawerSide::Bottom];
    for i in 0..sides.len() {
        for j in i+1..sides.len() {
            assert_ne!(sides[i].classes(), sides[j].classes());
        }
    }
}
