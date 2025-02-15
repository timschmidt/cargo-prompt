use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use rustminify::{remove_docs, minify_file};

/// A small CLI application that traverses a directory for `.rs` files,
/// optionally strips documentation, and minifies each file's contents.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "cargo-command")]
    command: String,

    /// Path to the directory to traverse
    #[arg(default_value = ".", value_name = "DIR")]
    dir: PathBuf,

    /// Remove documentation before minifying
    #[arg(short = 'r', long = "remove-docs")]
    remove_docs: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    
    // Attempt to load the project name from Cargo.toml
    let cargo_toml_path = args.dir.join("Cargo.toml");
    let project_name = if cargo_toml_path.exists() {
        let contents = fs::read_to_string(&cargo_toml_path)?;
        let parsed: toml::Value = toml::from_str(&contents)?;
        // Grab the name from [package] table or default if missing
        parsed
            .get("package")
            .and_then(|pkg| pkg.get("name"))
            .and_then(|name| name.as_str())
            .unwrap_or("Unnamed Project")
            .to_owned()
    } else {
        "Unnamed Project".to_string()
    };

    // We'll accumulate our output in a String, then print at the end
    let mut markdown_output = String::new();

    // Build a walker that respects .gitignore files by default
    let walker = WalkBuilder::new(&args.dir)
        .git_ignore(true)  // enable .gitignore parsing
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                // If it's a file and has .rs extension, process it
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                        match process_rust_file(path, args.remove_docs) {
                            Ok(minified) => {
                                markdown_output.push_str(&format!(
                                    "## {}\n\n```rust\n{}\n```\n\n",
                                    path.display(),
                                    minified
                                ));
                            }
                            Err(e) => {
                                eprintln!("Error processing {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                // If there's an error reading a directory entry, just print it
                eprintln!("Error reading directory entry: {}", e);
            }
        }
    }

    // Print the final markdown document to stdout
    println!("# {}\n", project_name);
    println!("{}", markdown_output);

    Ok(())
}

/// Reads a Rust file, optionally removes docs, minifies, and returns the minified string.
fn process_rust_file(path: &Path, strip_docs: bool) -> anyhow::Result<String> {
    let code = fs::read_to_string(path)?;
    let ast = syn::parse_file(&code)?;

    // If the user wants to remove docs, do so before minifying.
    let ast = if strip_docs {
        remove_docs(ast)
    } else {
        ast
    };

    // Minify the AST into a single-string representation
    let minified = minify_file(&ast);

    Ok(minified)
}
