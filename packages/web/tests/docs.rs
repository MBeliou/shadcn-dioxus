use web::docs::{get_all_components, component_exists};
use web::docs::loader::parse_doc;

#[test]
fn test_all_components_have_unique_slugs() {
    let components = get_all_components();
    let mut slugs: Vec<&str> = components.iter().map(|c| c.slug).collect();
    let len_before = slugs.len();
    slugs.sort_unstable();
    slugs.dedup();
    assert_eq!(len_before, slugs.len(), "Component slugs should be unique");
}

#[test]
fn test_component_exists_for_known_components() {
    assert!(component_exists("button"));
    assert!(component_exists("badge"));
    assert!(component_exists("accordion"));
}

#[test]
fn test_component_does_not_exist_for_unknown() {
    assert!(!component_exists("nonexistent"));
}

#[test]
fn test_parse_doc_for_button() {
    let markdown = web::docs::registry::get_component_doc("button");
    assert!(markdown.is_some(), "Button doc should exist");
    if let Some(md) = markdown {
        let parsed = parse_doc(md);
        assert!(parsed.is_some(), "Button doc should parse");
        if let Some(doc) = parsed {
            assert!(doc.frontmatter.title.contains("Button"));
            assert!(!doc.content.is_empty());
        }
    }
}

#[test]
fn test_all_registered_docs_parse() {
    let components = get_all_components();
    for comp in components {
        if let Some(md) = web::docs::registry::get_component_doc(comp.slug) {
            let parsed = parse_doc(md);
            assert!(parsed.is_some(), "Failed to parse doc for {}", comp.slug);
        }
    }
}
