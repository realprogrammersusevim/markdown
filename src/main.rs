use clap::{Arg, Command};
// use markdown::MarkdownParser;
//
// extern "C" {
//     fn tree_sitter_markdown() -> Language;
//     fn tree_sitter_markdown_inline() -> Language;
// }
//
// pub fn language() -> Language {
//     unsafe { tree_sitter_markdown() }
// }
//
// pub fn inline_language() -> Language {
//     unsafe { tree_sitter_markdown_inline() }
// }

fn main() {
    cli()
    // let mut parser = MarkdownParser::default();
    // let input = String::from("# Big header\n\nThis is another line\n\n- Bullet point");
    // let tree = parser.parse(&input.as_bytes(), None).unwrap();
    // println!("{:?}", tree)
}

fn cli() {
    Command::new("markdown")
        .author("Jonathan Milligan")
        .version("άλφα")
        .subcommand(
            Command::new("convention")
                .about("Change the file naming convention")
                .arg(
                    Arg::new("from")
                        .short('f')
                        .value_parser(["space", "camelCase", "snake_case"])
                        .default_value("space"),
                )
                .about("File naming convention that will be changed from")
                .arg(Arg::new("to").short('t').required(true).value_parser([
                    "space",
                    "camelCase",
                    "snake_case",
                ]))
                .about("File naming convention that will be changed to"),
        )
        .get_matches();
}
