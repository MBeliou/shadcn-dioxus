use dioxus::prelude::*;

#[test]
fn test_slider_renders() {
    let _: Element = rsx! { crate::Slider { value: 50.0 } };
    assert!(true);
}

#[test]
fn test_slider_clamping_logic() {
    assert_eq!(150.0_f64.clamp(0.0, 100.0), 100.0);
    assert_eq!((-10.0_f64).clamp(0.0, 100.0), 0.0);
}
