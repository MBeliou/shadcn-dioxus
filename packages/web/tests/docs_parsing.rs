use web::docs::loader::parse_doc;
use web::docs::registry::get_component_doc;
use web::docs::get_all_components;

#[test]
fn test_all_registry_docs_parse_successfully() {
    let components = get_all_components();
    for comp in components {
        if let Some(md) = get_component_doc(comp.slug) {
            let parsed = parse_doc(md);
            assert!(parsed.is_some(), "Failed to parse doc for '{}'", comp.slug);
            if let Some(doc) = parsed {
                // Title and description should be non‑empty
                assert!(!doc.frontmatter.title.is_empty());
                assert!(!doc.frontmatter.description.is_empty());
            }
        }
    }
}
