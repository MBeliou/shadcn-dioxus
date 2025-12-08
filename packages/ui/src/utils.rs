use crate::ItemChildProps;
use dioxus::prelude::*;
use tailwind_fuse::tw_merge;
/// Merges Tailwind CSS classes, resolving conflicts (later classes win).
/// This matches shadcn-svelte's `cn` function behavior.
pub fn cn(base: &str, additional: &str) -> String {
    if additional.is_empty() { base.to_string() } else { tw_merge!(base, additional) }
}
#[derive(Clone)]
pub struct RenderFn(pub fn(ItemChildProps, Element) -> Element);
impl PartialEq for RenderFn {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
impl RenderFn {
    pub fn new(f: fn(ItemChildProps, Element) -> Element) -> Option<Self> {
        Some(Self(f))
    }
}
