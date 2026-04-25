use crate::{InputType, input_classes};

#[test]
fn test_input_type_as_str() {
    assert_eq!(InputType::Text.as_str(), "text");
    assert_eq!(InputType::Password.as_str(), "password");
    assert_eq!(InputType::Email.as_str(), "email");
    assert_eq!(InputType::Number.as_str(), "number");
    assert_eq!(InputType::Tel.as_str(), "tel");
    assert_eq!(InputType::Url.as_str(), "url");
    assert_eq!(InputType::Search.as_str(), "search");
    assert_eq!(InputType::Date.as_str(), "date");
    assert_eq!(InputType::Time.as_str(), "time");
    assert_eq!(InputType::DatetimeLocal.as_str(), "datetime-local");
    assert_eq!(InputType::Month.as_str(), "month");
    assert_eq!(InputType::Week.as_str(), "week");
    assert_eq!(InputType::Color.as_str(), "color");
    assert_eq!(InputType::Hidden.as_str(), "hidden");
    assert_eq!(InputType::File.as_str(), "file");
}

#[test]
fn test_input_classes_text() {
    let result = input_classes(InputType::Text);
    assert!(result.contains("bg-background"));
    assert!(result.contains("dark:bg-input/30"));
    assert!(!result.contains("pt-1.5"));
}

#[test]
fn test_input_classes_file() {
    let result = input_classes(InputType::File);
    assert!(result.contains("bg-transparent"));
    assert!(result.contains("pt-1.5"));
}
