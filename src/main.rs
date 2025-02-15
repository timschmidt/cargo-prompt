// Cargo.toml (snippet of required dependencies)
//
// [dependencies]
// clap = "4.2"         # or the latest clap 4.x
// walkdir = "2.3"
// syn = "2.0"          # For syn::parse_file
// rustminify = "0.1"   # Example, actual version depends on your crate registry
// anyhow = "1.0"       # for easy error handling (optional)

use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// We assume `rustminify` provides these two functions:
/// - remove_docs(syn::File) -> syn::File
/// - minify_file(&syn::File) -> String
use rustminify::{remove_docs, minify_file};

/// A small CLI application that traverses a directory for `.rs` files,
/// optionally strips documentation, and minifies each file's contents.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the directory to traverse
    dir: PathBuf,

    /// Remove documentation before minifying
    #[arg(short = 'r', long = "remove-docs")]
    remove_docs: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // We'll accumulate our output in a String, then print at the end
    let mut markdown_output = String::new();

    // Traverse the directory recursively using walkdir
    for entry in WalkDir::new(&args.dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                // Process this .rs file
                match process_rust_file(path, args.remove_docs) {
                    Ok(minified) => {
                        // Append a markdown section for each file
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

    // Print the final markdown document to stdout
    println!("# Minified Rust Files\n");
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

