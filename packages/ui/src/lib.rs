//! This crate contains all shared UI for the workspace.
mod utils;
pub use utils::{cn, RenderFn};


mod button;
pub use button::*;

pub mod button_group;
pub use button_group as ButtonGroup;
//pub use button_group as ButtonGroup;

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