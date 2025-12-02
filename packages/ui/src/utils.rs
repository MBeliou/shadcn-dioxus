pub fn cn(base: &str, additional: &str) -> String {
    if additional.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, additional)
    }
}
