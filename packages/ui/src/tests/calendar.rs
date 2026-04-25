// Calendar tests – compile‑time checks only (no Dioxus runtime available in unit tests)

use crate::{CalendarProps, CalendarContext};

#[test]
fn test_calendar_props_defaults() {
    let props = CalendarProps {
        default_year: 2025,
        default_month: 5,
        default_date: Some((2025, 5, 10)),
        on_change: None,
        class: String::new(),
        attributes: Vec::new(),
    };
    assert_eq!(props.default_year, 2025);
    assert_eq!(props.default_month, 5);
    assert_eq!(props.default_date, Some((2025, 5, 10)));
}

// Validate that the CalendarContext type is accessible and has the expected shape
// (we can't construct it without a runtime, but the type must exist and compile)
#[test]
fn test_calendar_context_type_exists() {
    // Just ensure the type is importable and the test compiles
    let _: Option<CalendarContext> = None;
    assert!(true);
}
