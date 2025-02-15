# cargo-prompt
Collapse a rust project into a single minified markdown document for prompting.  Optionally remove documentation / comments.

## install

```shell
cargo install cargo-prompt
```

## run

```shell
cd my_cargo_project/
cargo prompt
```

## remove documentation

```shell
cd my_cargo_project/
cargo prompt -r
```

## redirect to a file

```shell
cd my_cargo_project/
cargo prompt > saved_prompt.txt
```
