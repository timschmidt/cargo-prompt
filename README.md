# cargo-prompt
Collapse a rust project into a single minified markdown document for prompting.  Optionally remove comments / documentation.  Items in .gitignore are automatically excluded.

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

## minify and include javascript as well as rust

```shell
cargo prompt -j
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
