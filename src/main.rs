use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_markdown() -> Language;
    fn tree_sitter_markdown_inline() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_markdown() }
}

pub fn inline_language() -> Language {
    unsafe { tree_sitter_markdown_inline() }
}

fn main() {
    let mut parser = Parser::new();
    parser.set_language(language()).unwrap();

    // Generate some test Markdown
    let markdown = "# Hello, world!

    Whatup world This is a list of all the things I love

    - Live
    - Laugh
    - Love";

    let tree = parser.parse(markdown, None).unwrap();

    println!("Tree: {:?}", tree.root_node().to_sexp())
}
