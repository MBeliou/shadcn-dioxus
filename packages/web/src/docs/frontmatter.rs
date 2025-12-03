use serde::Deserialize;
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct ComponentLinks {
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub doc: Option<String>,
    #[serde(default)]
    pub api: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DocFrontmatter {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub component: bool,
    #[serde(default)]
    pub links: Option<ComponentLinks>,
}
