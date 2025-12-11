//! This crate contains all shared UI for the workspace.
mod utils;
pub use utils::{cn, RenderFn};
mod button;
pub use button::*;
pub mod button_group;
pub use button_group as ButtonGroup;
pub mod spinner;
pub use spinner::*;
pub mod separator;
pub use separator::Separator;
pub mod item;
pub use item::*;
pub mod empty;
pub use empty::*;
pub mod card;
pub use card::*;
pub mod avatar;
pub use avatar::*;
pub mod skeleton;
pub use skeleton::*;
pub mod badge;
pub use badge::*;
pub mod input;
pub use input::*;
pub mod label;
pub use label::*;
pub mod progress;
pub use progress::*;
pub mod kbd;
pub use kbd::*;
pub mod checkbox;
pub use checkbox::*;
pub mod textarea;
pub use textarea::*;
pub mod alert;
pub use alert::*;
pub mod switch;
pub use switch::*;
pub mod field;
pub use field::*;
pub mod toggle;
pub use toggle::*;
pub mod aspect_ratio;
pub use aspect_ratio::*;
pub mod native_select;
pub use native_select::*;
pub mod input_group;
pub use input_group::*;
pub mod dialog;
pub use dialog::*;
pub mod portal;
pub use portal::*;
#[cfg(test)]
mod tests {
    use tailwind_fuse::tw_merge;
    #[test]
    fn test_ring_arbitrary_value_merge() {
        let result = tw_merge!("ring-[3px]", "ring-0");
        assert_eq!(result, "ring-0", "Arbitrary ring value should be overridden");
    }
    #[test]
    fn test_ring_arbitrary_value_with_variant() {
        let result = tw_merge!("focus-visible:ring-[3px]", "focus-visible:ring-0");
        assert_eq!(
            result,
            "focus-visible:ring-0",
            "Variant with arbitrary ring should be overridden",
        );
    }
    #[test]
    fn test_ring_width_and_inset_separate() {
        let result = tw_merge!("ring-1", "ring-inset");
        assert_eq!(
            result,
            "ring-1 ring-inset",
            "ring-width and ring-inset should not conflict",
        );
    }
}
