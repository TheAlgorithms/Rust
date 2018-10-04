# The Algorithms: Rust

This project aims at showcasing common algorithms implemented in `Rust`, with an accent on idiomatic code and genericity. 

## Project structure

The project is organized as follow:

`src/`
  - `my_algo_category/`
    - `mod.rs`
    - `my_algorithm.rs`
    - `some_other_algorithm.rs`
  - `some_other_algo_category/`
    - ...


`mod.rs` contains the export:

```rust
mod my_algorithm;

pub use self::my_algorithm::my_algorithm;
```

`my_algorithm.rs` contains your algorithm and the related tests:

```rust
pub fn my_algorithm() {
    // ...
}

#[cfg(test)]
mod tests {
    #[test]
    fn my_test() {
        // ...
    }
}
```

## Before submitting you PR

Make sure you:
  * ran `cargo fmt` and `cargo test`
  * **squashed** your branch !
