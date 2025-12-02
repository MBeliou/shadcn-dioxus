pub fn cn(base: &str, additional: Option<&str>) -> String {
    match additional {
        Some(extra) if !extra.is_empty() => format!("{} {}", base, extra),
        _ => base.to_string(),
    }
}
