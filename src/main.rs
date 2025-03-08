use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use rustminify::{remove_docs, minify_file};
use minify_js::{Session, TopLevelMode, minify};

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
    
    /// Also minify .js files
    #[arg(short = 'j', long = "javascript")]
    javascript: bool,

    /// Also minify .py, pyw files
    #[arg(short = 'p', long = "python")]
    python: bool,
    
    /// Also minify .java files
    #[arg(long = "java")]
    java: bool,
    
    /// Also minify .c / .cpp files
    #[arg(short = 'c', long = "c-cpp")]
    cpp: bool,
    
    /// Also minify .csharp files
    #[arg(short = 'i', long = "csharp")]
    csharp: bool,
    
    /// Also minify .php files
    #[arg(short = 'q', long = "php")]
    php: bool,
    
    /// Also minify .rb files
    #[arg(long = "ruby")]
    ruby: bool,
    
    /// Also minify .swift files
    #[arg(short = 's', long = "swift")]
    swift: bool,
    
    /// Also minify .ts files
    #[arg(short = 't', long = "typescript")]
    typescript: bool,
    
    /// Also minify .kt files
    #[arg(short = 'k', long = "kotlin")]
    kotlin: bool,
    
    /// Also minify .go files
    #[arg(short = 'g', long = "go")]
    go: bool,
    
    /// Also minify .r files
    #[arg(long = "r")]
    r: bool,
    
    /// Also minify .m files
    #[arg(short = 'm', long = "matlab")]
    matlab: bool,
    
    /// Also minify .vb files
    #[arg(short = 'v', long = "vbnet")]
    vbnet: bool,
    
    /// Also minify .pl files
    #[arg(long = "perl")]
    perl: bool,
    
    /// Also minify .scala files
    #[arg(long = "scala")]
    scala: bool,
    
    /// Also minify .dart files
    #[arg(short = 'd', long = "dart")]
    dart: bool,
    
    /// Also minify .groovy files
    #[arg(long = "groovy")]
    groovy: bool,
    
    /// Also minify .jl files
    #[arg(long = "julia")]
    julia: bool,
    
    /// Also minify .hs files
    #[arg(long = "haskell")]
    haskell: bool,
    
    /// Also minify .sh files
    #[arg(long = "shell")]
    shell: bool,
    
    /// Also minify .lua files
    #[arg(short = 'l', long = "lua")]
    lua: bool,
    
    /// Minify all supported languages
    #[arg(short = 'a', long = "all")]
    all: bool,
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
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    let path = entry.path();
                    // Process Rust files
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
                    
                    // Process JavaScript files (if the flag is set)
                    if (args.javascript || args.all) && path.extension().and_then(|s| s.to_str()) == Some("js") {
                        match process_javascript_file(path, args.remove_docs) {
                            Ok(minified) => {
                                markdown_output.push_str(&format!(
                                    "## {}\n\n```javascript\n{}\n```\n\n",
                                    path.display(),
                                    minified
                                ));
                            }
                            Err(e) => {
                                eprintln!("Error processing {}: {}", path.display(), e);
                            }
                        }
                    }
                    
                    // Python
                    if (args.python || args.all) && (path.extension().and_then(|s| s.to_str()) == Some("py") || path.extension().and_then(|s| s.to_str()) == Some("pyw")) {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["__pycache__".to_string(), "venv".to_string(), ".env".to_string(), "dist".to_string()];
                        let line_comment = "#".to_string();
                        let block_comment_start = "'''".to_string();
                        let block_comment_end = "'''".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(&file_contents, &line_comment, &block_comment_start, &block_comment_end)
                        } else {
                            file_contents
                        };
                        
                        let minified = remove_whitespace(&stripped);
                        
                        markdown_output.push_str(&format!(
                            "## {}\n\n```python\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Java
                    if (args.java || args.all) && path.extension().and_then(|s| s.to_str()) == Some("java") {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["target".to_string(), "build".to_string(), "out".to_string()];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(&file_contents, &line_comment, &block_comment_start, &block_comment_end)
                        } else {
                            file_contents
                        };
                        
                        let minified = remove_whitespace(&stripped);
                        
                        markdown_output.push_str(&format!(
                            "## {}\n\n```java\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // C / C++
                    if (args.cpp || args.all) && 
                        (
                        path.extension().and_then(|s| s.to_str()) == Some("cpp") || 
                        path.extension().and_then(|s| s.to_str()) == Some("hpp") ||
                        path.extension().and_then(|s| s.to_str()) == Some("cc") ||
                        path.extension().and_then(|s| s.to_str()) == Some("hh") ||
                        path.extension().and_then(|s| s.to_str()) == Some("cxx") ||
                        path.extension().and_then(|s| s.to_str()) == Some("hxx") ||
                        path.extension().and_then(|s| s.to_str()) == Some("c") ||
                        path.extension().and_then(|s| s.to_str()) == Some("h") ||
                        path.extension().and_then(|s| s.to_str()) == Some("m") ||
                        path.extension().and_then(|s| s.to_str()) == Some("mm")
                        ) {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["build".to_string(), "obj".to_string(), "bin".to_string()];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(&file_contents, &line_comment, &block_comment_start, &block_comment_end)
                        } else {
                            file_contents
                        };
                        
                        let minified = remove_whitespace(&stripped);
                        
                        markdown_output.push_str(&format!(
                            "## {}\n\n```c/c++/obj-c\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // C#
                    if (args.csharp || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("cs"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec![
                            "bin".to_string(),
                            "obj".to_string(),
                            "Debug".to_string(),
                            "Release".to_string(),
                        ];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```csharp\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // PHP
                    if (args.php || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("php"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["vendor".to_string(), "cache".to_string()];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```php\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Ruby
                    if (args.ruby || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("rb"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["vendor".to_string(), "tmp".to_string(), "log".to_string()];
                        let line_comment = "#".to_string();
                        let block_comment_start = "=begin".to_string();
                        let block_comment_end = "=end".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```ruby\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Swift
                    if (args.swift || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("swift"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec![".build".to_string(), "Pods".to_string()];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```swift\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // TypeScript
                    if (args.typescript || args.all)
                        && (
                            path.extension().and_then(|s| s.to_str()) == Some("ts")
                            || path.extension().and_then(|s| s.to_str()) == Some("tsx")
                        )
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec![
                            "node_modules".to_string(),
                            "dist".to_string(),
                            "build".to_string(),
                        ];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```typescript\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Kotlin
                    if (args.kotlin || args.all)
                        && (
                            path.extension().and_then(|s| s.to_str()) == Some("kt")
                            || path.extension().and_then(|s| s.to_str()) == Some("kts")
                        )
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["build".to_string(), "out".to_string()];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```kotlin\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Go
                    if (args.go || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("go"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["vendor".to_string(), "bin".to_string()];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```go\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // R
                    if (args.r || args.all)
                        && (
                            path.extension().and_then(|s| s.to_str()) == Some("r")
                            || path.extension().and_then(|s| s.to_str()) == Some("R")
                        )
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["renv".to_string()];
                        let line_comment = "#".to_string();
                        // R doesn't truly have traditional block comments
                        let block_comment_start = "".to_string();
                        let block_comment_end = "".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```r\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // MATLAB
                    if (args.matlab || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("m"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["bin".to_string()];
                        let line_comment = "%".to_string();
                        let block_comment_start = "%{".to_string();
                        let block_comment_end = "%}".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```matlab\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // VB.NET
                    if (args.vbnet || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("vb"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["bin".to_string(), "obj".to_string()];
                        let line_comment = "'".to_string();
                        // VB.NET uses line comments primarily
                        let block_comment_start = "".to_string();
                        let block_comment_end = "".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```vbnet\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Scala
                    if (args.scala || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("scala"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["target".to_string(), "project/target".to_string()];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```scala\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Perl
                    if (args.perl || args.all)
                        && (
                            path.extension().and_then(|s| s.to_str()) == Some("pl")
                            || path.extension().and_then(|s| s.to_str()) == Some("pm")
                        )
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["blib".to_string(), "_build".to_string()];
                        let line_comment = "#".to_string();
                        let block_comment_start = "=pod".to_string();
                        let block_comment_end = "=cut".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```perl\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Dart
                    if (args.dart || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("dart"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["build".to_string(), ".dart_tool".to_string()];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```dart\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Groovy
                    if (args.groovy || args.all)
                        && (
                            path.extension().and_then(|s| s.to_str()) == Some("groovy")
                            || path.extension().and_then(|s| s.to_str()) == Some("gvy")
                            || path.extension().and_then(|s| s.to_str()) == Some("gy")
                            || path.extension().and_then(|s| s.to_str()) == Some("gsh")
                        )
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["target".to_string(), "build".to_string()];
                        let line_comment = "//".to_string();
                        let block_comment_start = "/*".to_string();
                        let block_comment_end = "*/".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```groovy\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Julia
                    if (args.julia || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("jl"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["docs/build".to_string()];
                        let line_comment = "#".to_string();
                        let block_comment_start = "#=".to_string();
                        let block_comment_end = "=#".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```julia\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Haskell
                    if (args.haskell || args.all)
                        && (
                            path.extension().and_then(|s| s.to_str()) == Some("hs")
                            || path.extension().and_then(|s| s.to_str()) == Some("lhs")
                        )
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["dist".to_string(), ".stack-work".to_string()];
                        let line_comment = "--".to_string();
                        let block_comment_start = "{-".to_string();
                        let block_comment_end = "-}".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```haskell\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Shell/Bash
                    if (args.shell || args.all)
                        && (
                            path.extension().and_then(|s| s.to_str()) == Some("sh")
                            || path.extension().and_then(|s| s.to_str()) == Some("bash")
                        )
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["tmp".to_string()];
                        let line_comment = "#".to_string();
                        // Shell typically uses only line comments
                        let block_comment_start = "".to_string();
                        let block_comment_end = "".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```bash\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
                    }
                    
                    // Lua
                    if (args.lua || args.all)
                        && (path.extension().and_then(|s| s.to_str()) == Some("lua"))
                    {
                        let file_contents = fs::read_to_string(path)?;
                        let _default_skip_dirs = vec!["bin".to_string()];
                        let line_comment = "--".to_string();
                        let block_comment_start = "--[[".to_string();
                        let block_comment_end = "]]".to_string();
                    
                        let stripped = if args.remove_docs {
                            remove_documentation(
                                &file_contents,
                                &line_comment,
                                &block_comment_start,
                                &block_comment_end,
                            )
                        } else {
                            file_contents
                        };
                    
                        let minified = remove_whitespace(&stripped);
                    
                        markdown_output.push_str(&format!(
                            "## {}\n\n```lua\n{}\n```\n\n",
                            path.display(),
                            minified
                        ));
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

/// Reads a javascript file, optionally removes docs, minifies, and returns the minified string.
fn process_javascript_file(path: &Path, strip_docs: bool) -> anyhow::Result<String> {
    let code = fs::read_to_string(path)?;

    // If the user wants to remove docs, do so before minifying.
    if strip_docs {
        
    } else {
        
    };

    let session = Session::new();
    let mut out = Vec::new();
    
    // Minify the javascript into a single-string representation
    minify(&session, TopLevelMode::Global, code.as_bytes(), &mut out).unwrap();

    // Convert the resulting Vec<u8> to a String
    let minified = String::from_utf8(out)?;

    Ok(minified)
}

/// Remove line and block comments from the string, preserving everything else (including whitespace).
///
/// - `line_comment` is something like "#" or "//"
/// - `block_comment_start` is something like "/*" or "'''"
/// - `block_comment_end` is something like "*/" or "'''"
fn remove_documentation(
    content: &str,
    line_comment: &str,
    block_comment_start: &str,
    block_comment_end: &str,
) -> String {
    let mut result = String::new();

    let mut in_string = false;
    let mut in_char = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    let mut prev_char = None;
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        // If we're in a line comment, consume until newline
        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
                // Keep the newline
                result.push(c);
            }
            prev_char = Some(c);
            continue;
        }

        // If we're in a block comment, look for the block_comment_end pattern
        if in_block_comment {
            // Check if we've hit the end of a block comment
            if c == block_comment_end.chars().next().unwrap() {
                let mut is_block_end = true;
                for expected in block_comment_end.chars().skip(1) {
                    if chars.next() != Some(expected) {
                        is_block_end = false;
                        break;
                    }
                }
                if is_block_end {
                    in_block_comment = false;
                }
            }
            prev_char = Some(c);
            continue;
        }

        // Handle string toggling
        match c {
            '"' if !in_char => {
                // Toggle string if not escaped
                if prev_char != Some('\\') {
                    in_string = !in_string;
                }
                result.push(c);
            }
            '\'' if !in_string => {
                // Toggle char literal if not escaped
                if prev_char != Some('\\') {
                    in_char = !in_char;
                }
                result.push(c);
            }
            _ => {
                // If not in a string or char, check if this is the start of a comment
                if !in_string && !in_char {
                    // Check for line comment
                    if c == line_comment.chars().next().unwrap() {
                        let mut is_line = true;
                        for expected in line_comment.chars().skip(1) {
                            if chars.next() != Some(expected) {
                                is_line = false;
                                break;
                            }
                        }
                        if is_line {
                            in_line_comment = true;
                            prev_char = Some(c);
                            continue;
                        } else {
                            // Not actually a comment, so push the character we saw + any consumed
                            result.push(c);
                            prev_char = Some(c);
                            continue;
                        }
                    }

                    // Check for block comment
                    if c == block_comment_start.chars().next().unwrap() {
                        let mut is_block = true;
                        for expected in block_comment_start.chars().skip(1) {
                            if chars.next() != Some(expected) {
                                is_block = false;
                                break;
                            }
                        }
                        if is_block {
                            in_block_comment = true;
                            prev_char = Some(c);
                            continue;
                        } else {
                            // Not actually a block comment, push char + any consumed
                            result.push(c);
                            prev_char = Some(c);
                            continue;
                        }
                    }
                }

                // Otherwise, just push the character
                result.push(c);
            }
        }

        prev_char = Some(c);
    }

    result
}

/// Remove extra whitespace, newlines, and other “non-code” spacing outside of string/char literals.
fn remove_whitespace(content: &str) -> String {
    let mut result = String::new();

    let mut in_string = false;
    let mut in_char = false;
    let mut prev_char = None;
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // Toggle string if not escaped
            '"' => {
                if prev_char != Some('\\') && !in_char {
                    in_string = !in_string;
                }
                result.push(c);
            }
            // Toggle char literal if not escaped
            '\'' => {
                if prev_char != Some('\\') && !in_string {
                    in_char = !in_char;
                }
                result.push(c);
            }
            '\n' | '\r' | '\t' | ' ' => {
                // If we're inside a string/char, keep whitespace (for correctness of literal).
                // Otherwise, skip it.
                if in_string || in_char {
                    if c == '\n' || c == '\r' {
                        // Convert newlines inside string to \n (optional).
                        result.push_str("\\n");
                    } else {
                        // Keep the space or tab inside the literal
                        result.push(c);
                    }
                }
            }
            '\\' => {
                // If we're in a string, we need to handle escapes
                if in_string || in_char {
                    // Push backslash
                    result.push(c);
                    // If next char is an escapable character, push it too
                    if let Some(&next) = chars.peek() {
                        if matches!(next, 'n' | 'r' | 't' | '\\' | '"' | '\'') {
                            result.push(chars.next().unwrap());
                        }
                    }
                } else {
                    // If outside a string, we typically just skip or handle. Keep it if you want.
                    // In many languages a backslash outside string might not be meaningful,
                    // but let's preserve it:
                    result.push(c);
                }
            }
            _ => {
                // Normal character
                result.push(c);
            }
        }
        prev_char = Some(c);
    }

    // As a final optional step, you could do something like:
    // result.split_whitespace().collect::<Vec<_>>().join(" ")
    // but that might destroy spacing in string literals, so be careful.

    result
}
