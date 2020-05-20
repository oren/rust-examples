# Tiny Markdown Compiler

Code for [markdown compiler tutorial](https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler)

## Chapter 2 - Functions

Code
```Rust
fn get_version() -> u16 {
    1000
}

fn usage() {
    let the_version = get_version();
    println!("tinymd, a markdown compiler written by Oren");
    println!("Version {}", the_version);
}

fn main() {
    usage();
}
```

Run
```
cargo run -q

=>

tinymd, a markdown compiler written by Oren
Version 1000
```
