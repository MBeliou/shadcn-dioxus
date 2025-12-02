use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SeparatorOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl SeparatorOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }

    pub fn base_class(&self) -> &'static str {
        match self {
            Self::Vertical => "w-px",
            Self::Horizontal => "h-px w-full",
        }
    }
}

const SEPARATOR_BASE: &str = "bg-border shrink-0";
const BUTTON_GROUP_SEPARATOR_CLASSES: &str = "bg-input relative !m-0 self-stretch data-[orientation=vertical]:h-auto";

fn cn(base: &str, additional: Option<&str>) -> String {
    match additional {
        Some(extra) if !extra.is_empty() => format!("{} {}", base, extra),
        _ => base.to_string(),
    }
}

#[component]
pub fn ButtonGroupSeparator(
    #[props(default)] orientation: SeparatorOrientation,
    class: Option<String>,
    #[props(default = true)] decorative: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base_classes = format!(
        "{} {} {}",
        SEPARATOR_BASE,
        orientation.base_class(),
        BUTTON_GROUP_SEPARATOR_CLASSES
    );
    let classes = cn(&base_classes, class.as_deref());

    let aria_orientation = if orientation == SeparatorOrientation::Vertical {
        Some("vertical")
    } else {
        None
    };

    rsx! {
        div {
            "data-slot": "button-group-separator",
            "data-orientation": orientation.as_str(),
            role: if !decorative { "separator" } else { "none" },
            "aria-orientation": aria_orientation,
            class: "{classes}",
            ..attributes,
        }
    }
}