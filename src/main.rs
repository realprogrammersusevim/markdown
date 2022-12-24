use clap::{Parser, Subcommand};
mod convention;
mod file_management;
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

#[derive(Parser)]
#[command(name = "markdown")]
#[command(about = "A command line tool for parsing and converting markdown files.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Change file naming conventions
    #[command(arg_required_else_help = true)]
    Convention {
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
        #[arg(short, long, default_value = ".")]
        path: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Convention { from, to, path } => {
            convention::convention(from, to, path);
        }
    }
    // let mut parser = MarkdownParser::default();
    // let input = String::from("# Big header\n\nThis is another line\n\n- Bullet point");
    // let tree = parser.parse(&input.as_bytes(), None).unwrap();
    // println!("{:?}", tree)
}
