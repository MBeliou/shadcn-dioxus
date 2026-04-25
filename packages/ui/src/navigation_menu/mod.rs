mod navigation_menu;
mod navigation_menu_content;
mod navigation_menu_indicator;
mod navigation_menu_item;
mod navigation_menu_link;
mod navigation_menu_list;
mod navigation_menu_trigger;

pub use navigation_menu::*;
pub use navigation_menu_content::*;
pub use navigation_menu_indicator::*;
pub use navigation_menu_item::*;
pub use navigation_menu_link::*;
pub use navigation_menu_list::*;
pub use navigation_menu_trigger::*;

/// Utility function for styling links that don't have a content dropdown
pub fn navigation_menu_trigger_style() -> &'static str {
    "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[state=open]:bg-accent/50"
}
