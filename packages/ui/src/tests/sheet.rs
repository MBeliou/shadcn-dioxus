use crate::Side;

#[test]
fn test_sheet_side_as_str() {
    assert_eq!(Side::Top.as_str(), "top");
    assert_eq!(Side::Bottom.as_str(), "bottom");
    assert_eq!(Side::Left.as_str(), "left");
    assert_eq!(Side::Right.as_str(), "right");
}

#[test]
fn test_sheet_side_class() {
    let right = Side::Right.class();
    assert!(right.contains("end-0"));
    assert!(right.contains("border-s"));
    let left = Side::Left.class();
    assert!(left.contains("start-0"));
    assert!(left.contains("border-e"));
}

#[test]
fn test_sheet_side_all_distinct_classes() {
    let sides = [Side::Top, Side::Bottom, Side::Left, Side::Right];
    for i in 0..sides.len() {
        for j in i+1..sides.len() {
            assert_ne!(sides[i].class(), sides[j].class());
        }
    }
}
