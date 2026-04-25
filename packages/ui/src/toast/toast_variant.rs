use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ToastVariant {
    #[default]
    Default,
    Destructive,
    Success,
}

impl ToastVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
            Self::Success => "success",
        }
    }
}
