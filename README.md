# cargo-prompt
Recursively minify and concatenate source code into a markdown document for llm prompting.  Optionally remove comments / documentation.  Items in .gitignore are automatically excluded.

This can be useful for LLM assisted development when rapidly prompting with updated code, or when there is too much code to fit in the allowed context window.

## install

```shell
cargo install cargo-prompt
```

## run

```shell
cd my_cargo_project/
cargo prompt
```

## remove comments / documentation

```shell
cd my_cargo_project/
cargo prompt -r
```

## redirect to a file

```shell
cd my_cargo_project/
cargo prompt > saved_prompt.txt
```

## target specific directory

```shell
cargo prompt /path/to/src/
```

## other languages

### javascript
```shell
cargo prompt -j
cargo prompt --javascript
```

### python
```shell
cargo prompt -p
cargo prompt --python
```

### java
```shell
cargo prompt --java
```

### c / c++
```shell
cargo prompt -c
cargo prompt --c-cpp
```

### csharp
```shell
cargo prompt -i
cargo prompt --csharp
```

### php
```shell
cargo prompt -q
cargo prompt --php
```

### ruby
```shell
cargo prompt --ruby
```

### swift
```shell
cargo prompt -s
cargo prompt --swift
```

### typescript
```shell
cargo prompt -t
cargo prompt --typescript
```

### kotlin
```shell
cargo prompt -k
cargo prompt --kotlin
```

### go
```shell
cargo prompt -g
cargo prompt --go
```

### r
```shell
cargo prompt --r
```

### matlab
```shell
cargo prompt -m
cargo prompt --matlab
```

### vb
```shell
cargo prompt -v
cargo prompt --vbnet
```

### perl
```shell
cargo prompt --perl
```

### scala
```shell
cargo prompt --scala
```

### dart
```shell
cargo prompt -d
cargo prompt --dart
```

### groovy
```shell
cargo prompt --groovy
```

### julia
```shell
cargo prompt --julia
```

### haskell
```shell
cargo prompt --haskell
```

### shell
```shell
cargo prompt --shell
```

### lua
```shell
cargo prompt -l
cargo prompt --lua
```

## all languages
```shell
cargo prompt -a
cargo prompt --all
```

## example input
fizzbuzz/fizzbuzz.rs:
```rust
// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}
```

## example output
````markdown
# fizzbuzz

## ./fizzbuzz.rs

```rust
fn fizzbuzz(n:u32)->(){if is_divisible_by(n,15){println!("fizzbuzz");}else if is_divisible_by(n,3){println!("fizz");}else if is_divisible_by(n,5){println!("buzz");}else{println!("{}",n);}}
```
````

## todo
- https://crates.io/crates/llm-minify
