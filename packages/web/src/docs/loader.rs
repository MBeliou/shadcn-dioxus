use super::frontmatter::DocFrontmatter;
use gray_matter::{engine::YAML, Matter, ParsedEntity};
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedDoc {
    pub frontmatter: DocFrontmatter,
    pub content: String,
}
/**
 * Parses markdown content and splits it between frontmatter and actual content
 */
pub fn parse_doc(raw_markdown: &str) -> Option<ParsedDoc> {
    let matter = Matter::<YAML>::new();
    let result: ParsedEntity<DocFrontmatter> = matter.parse(raw_markdown).ok()?;
    Some(ParsedDoc {
        frontmatter: result.data?,
        content: result.content,
    })
}

