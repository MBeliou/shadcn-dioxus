pub fn get_component_doc(name: &str) -> Option<&'static str> {
    match name {
        "button" => Some(include_str!("../content/components/button.md")),
        _ => None,
    }
}
