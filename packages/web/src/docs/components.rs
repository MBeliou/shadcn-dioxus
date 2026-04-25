#[derive(Debug, Clone, PartialEq)]
pub struct ComponentMeta {
    pub slug: &'static str,
    pub title: &'static str,
    pub new: bool,
}

const COMPONENTS: &[ComponentMeta] = &[
    ComponentMeta { slug: "accordion",         title: "Accordion",         new: true },
    ComponentMeta { slug: "alert",             title: "Alert",             new: false },
    ComponentMeta { slug: "alert_dialog",      title: "Alert Dialog",      new: true },
    ComponentMeta { slug: "aspect-ratio",      title: "Aspect Ratio",      new: false },
    ComponentMeta { slug: "avatar",            title: "Avatar",            new: false },
    ComponentMeta { slug: "badge",             title: "Badge",             new: false },
    ComponentMeta { slug: "breadcrumb",        title: "Breadcrumb",        new: true },
    ComponentMeta { slug: "button",            title: "Button",            new: false },
    ComponentMeta { slug: "button-group",      title: "Button Group",      new: false },
    ComponentMeta { slug: "calendar",          title: "Calendar",          new: true },
    ComponentMeta { slug: "card",              title: "Card",              new: false },
    ComponentMeta { slug: "carousel",          title: "Carousel",          new: true },
    ComponentMeta { slug: "chart",             title: "Chart",             new: true },
    ComponentMeta { slug: "checkbox",          title: "Checkbox",          new: false },
    ComponentMeta { slug: "collapsible",       title: "Collapsible",       new: true },
    ComponentMeta { slug: "color_picker",      title: "Color Picker",      new: true },
    ComponentMeta { slug: "combobox",          title: "Combobox",          new: true },
    ComponentMeta { slug: "command",           title: "Command",           new: true },
    ComponentMeta { slug: "context_menu",      title: "Context Menu",      new: true },
    ComponentMeta { slug: "data_table",        title: "Data Table",        new: true },
    ComponentMeta { slug: "date_picker",       title: "Date Picker",       new: true },
    ComponentMeta { slug: "dialog",            title: "Dialog",            new: false },
    ComponentMeta { slug: "drawer",            title: "Drawer",            new: true },
    ComponentMeta { slug: "dropdown_menu",     title: "Dropdown Menu",     new: true },
    ComponentMeta { slug: "empty",             title: "Empty",             new: false },
    ComponentMeta { slug: "field",             title: "Field",             new: false },
    ComponentMeta { slug: "hover_card",        title: "Hover Card",        new: true },
    ComponentMeta { slug: "input",             title: "Input",             new: false },
    ComponentMeta { slug: "input-group",       title: "Input Group",       new: false },
    ComponentMeta { slug: "input_otp",         title: "Input OTP",         new: true },
    ComponentMeta { slug: "item",              title: "Item",              new: false },
    ComponentMeta { slug: "kbd",               title: "Kbd",               new: false },
    ComponentMeta { slug: "label",             title: "Label",             new: false },
    ComponentMeta { slug: "menubar",           title: "Menubar",           new: true },
    ComponentMeta { slug: "native-select",     title: "Native Select",     new: false },
    ComponentMeta { slug: "navigation_menu",   title: "Navigation Menu",   new: true },
    ComponentMeta { slug: "pagination",        title: "Pagination",        new: true },
    ComponentMeta { slug: "popover",           title: "Popover",           new: true },
    ComponentMeta { slug: "radio_group",       title: "Radio Group",       new: true },
    ComponentMeta { slug: "resizable",         title: "Resizable",         new: true },
    ComponentMeta { slug: "scroll_area",       title: "Scroll Area",       new: true },
    ComponentMeta { slug: "select",            title: "Select",            new: true },
    ComponentMeta { slug: "separator",         title: "Separator",         new: false },
    ComponentMeta { slug: "sheet",             title: "Sheet",             new: false },
    ComponentMeta { slug: "sidebar",           title: "Sidebar",           new: true },
    ComponentMeta { slug: "skeleton",          title: "Skeleton",          new: false },
    ComponentMeta { slug: "slider",            title: "Slider",            new: true },
    ComponentMeta { slug: "spinner",           title: "Spinner",           new: false },
    ComponentMeta { slug: "stepper",           title: "Stepper",           new: true },
    ComponentMeta { slug: "switch",            title: "Switch",            new: false },
    ComponentMeta { slug: "table",             title: "Table",             new: true },
    ComponentMeta { slug: "tabs",              title: "Tabs",              new: true },
    ComponentMeta { slug: "textarea",          title: "Textarea",          new: false },
    ComponentMeta { slug: "toggle",            title: "Toggle",            new: false },
    ComponentMeta { slug: "toggle_group",      title: "Toggle Group",      new: true },
    ComponentMeta { slug: "tooltip",           title: "Tooltip",           new: true },
];

pub fn get_all_components() -> &'static [ComponentMeta] {
    COMPONENTS
}

pub fn component_exists(slug: &str) -> bool {
    COMPONENTS.iter().any(|c| c.slug == slug)
}
