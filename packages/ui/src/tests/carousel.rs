use crate::CarouselOrientation;

#[test]
fn test_carousel_orientation_as_str() {
    assert_eq!(CarouselOrientation::Horizontal.as_str(), "horizontal");
    assert_eq!(CarouselOrientation::Vertical.as_str(), "vertical");
}
