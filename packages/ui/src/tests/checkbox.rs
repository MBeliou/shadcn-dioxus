use crate::CheckboxState;

#[test]
fn test_checkbox_default_is_unchecked() {
    assert_eq!(CheckboxState::default(), CheckboxState::Unchecked);
}

#[test]
fn test_checkbox_not_operator() {
    assert_eq!(!CheckboxState::Checked, CheckboxState::Unchecked);
    assert_eq!(!CheckboxState::Unchecked, CheckboxState::Checked);
    assert_eq!(!CheckboxState::Indeterminate, CheckboxState::Unchecked);
}

#[test]
fn test_checkbox_into_bool() {
    let checked: bool = CheckboxState::Checked.into();
    assert!(checked);
    let unchecked: bool = CheckboxState::Unchecked.into();
    assert!(!unchecked);
    let indeterminate: bool = CheckboxState::Indeterminate.into();
    assert!(indeterminate);
}
