use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SidebarContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
    pub open_mobile: Signal<bool>,
    pub set_open_mobile: Callback<bool>,
    pub variant: SidebarVariant,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

impl SidebarVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sidebar => "sidebar",
            Self::Floating => "floating",
            Self::Inset => "inset",
        }
    }
}
