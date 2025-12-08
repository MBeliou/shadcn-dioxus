#[derive(Debug, Clone, PartialEq)]
pub struct ComponentMeta {
    pub slug: &'static str,
    pub title: &'static str,
}
/// All available components. Components that do not exist are handled as well with a dedicated 404 page
const COMPONENTS: &[ComponentMeta] = &[
    ComponentMeta {
        slug: "avatar",
        title: "Avatar",
    },
    ComponentMeta {
        slug: "badge",
        title: "Badge",
    },
    ComponentMeta {
        slug: "button",
        title: "Button",
    },
    ComponentMeta {
        slug: "button-group",
        title: "Button Group",
    },
    ComponentMeta {
        slug: "card",
        title: "Card",
    },
    ComponentMeta {
        slug: "checkbox",
        title: "Checkbox",
    },
    ComponentMeta {
        slug: "empty",
        title: "Empty",
    },
    ComponentMeta {
        slug: "field",
        title: "Field",
    },
    ComponentMeta {
        slug: "input",
        title: "Input",
    },
    ComponentMeta {
        slug: "item",
        title: "Item",
    },
    ComponentMeta {
        slug: "kbd",
        title: "Kbd",
    },
    ComponentMeta {
        slug: "label",
        title: "Label",
    },
    ComponentMeta {
        slug: "separator",
        title: "Separator",
    },
    ComponentMeta {
        slug: "skeleton",
        title: "Skeleton",
    },
    ComponentMeta {
        slug: "spinner",
        title: "Spinner",
    },
    ComponentMeta {
        slug: "switch",
        title: "Switch",
    },
    ComponentMeta {
        slug: "textarea",
        title: "Textarea",
    },
    ComponentMeta {
        slug: "toggle",
        title: "Toggle",
    },
    ComponentMeta {
        slug: "alert",
        title: "Alert",
    },
];
pub fn get_all_components() -> &'static [ComponentMeta] {
    COMPONENTS
}
pub fn component_exists(slug: &str) -> bool {
    COMPONENTS.iter().any(|c| c.slug == slug)
}
