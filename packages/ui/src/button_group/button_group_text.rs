use dioxus::prelude::*;
const BASE_CLASSES: &str = "bg-muted shadow-xs flex items-center gap-2 rounded-md border px-4 text-sm font-medium [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none";
fn cn(base: &str, additional: Option<&str>) -> String {
    match additional {
        Some(extra) if !extra.is_empty() => format!("{} {}", base, extra),
        _ => base.to_string(),
    }
}
#[component]
pub fn ButtonGroupText(
    class: Option<String>,
    children: Element,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let classes = cn(BASE_CLASSES, class.as_deref());
    rsx! {
        div { "data-slot": "button-group-text", class: "{classes}", ..attributes, {children} }
    }
}
