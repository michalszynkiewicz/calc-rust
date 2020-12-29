# Simple calculator

Reverse Polish Notation algorithm implementation in Rust.

## Building
Be sure you have Rust installed.

Run the following command in the project's root directory:
```shell
cargo build --release
```

`cargo` creates a binary called `calc` in the `target/release` directory.

## Running
To calculate a value of an expression with it run `calc` followed 
by the expression 
```shell
$ calc '(1 - -1)/ 2 * 12'
12.0 
```