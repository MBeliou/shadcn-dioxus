use crate::{InputType, input_classes};
use proptest::prelude::*;

proptest! {
    #[test]
    fn input_type_always_has_classes(type_idx in 0usize..15) {
        let types = [
            InputType::Text, InputType::Password, InputType::Email, InputType::Number,
            InputType::Tel, InputType::Url, InputType::Search, InputType::Date,
            InputType::Time, InputType::DatetimeLocal, InputType::Month, InputType::Week,
            InputType::Color, InputType::Hidden, InputType::File,
        ];
        let t = types[type_idx];
        let classes = input_classes(t);
        assert!(classes.contains("flex"));
        assert!(!classes.is_empty());
    }
}
