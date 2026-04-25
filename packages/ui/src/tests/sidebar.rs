use crate::SidebarVariant;

#[test]
fn test_sidebar_variant_as_str() {
    assert_eq!(SidebarVariant::Sidebar.as_str(), "sidebar");
    assert_eq!(SidebarVariant::Floating.as_str(), "floating");
    assert_eq!(SidebarVariant::Inset.as_str(), "inset");
}

#[test]
fn test_sidebar_variants_distinct() {
    let variants = [SidebarVariant::Sidebar, SidebarVariant::Floating, SidebarVariant::Inset];
    for i in 0..variants.len() {
        for j in i+1..variants.len() {
            assert_ne!(variants[i].as_str(), variants[j].as_str());
        }
    }
}
