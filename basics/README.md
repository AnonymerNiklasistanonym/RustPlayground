# Rust

This document tracks information about Rust since this is my first project written in that language.

## Setup

1. Install `rustup` via the official package manager

   ```sh
   pacman -S rustup
   ```

2. Download the latest stable release of Rust using `rustup`

   ```sh
   rustup default stable
   # optional update it to be the latest version
   rustup update stable
   ```

3. Create project in a new directory `project-name`

    ```sh
    cargo new project-name
    ```

    ```
    ├── Cargo.toml
    └── src
        └── main.rs
    ```

    Alternativley the current directory can be initalized:

    ```
    cargo init
    ```

### Install dependencies

```sh
cargo add tokio
cargo remove tokio
# update dependencies
cargo update
```

Dependencies are being tracked in `Cargo.toml` (with detailed information stored in `Cargo.lock`).

### Build/Run

```sh
# dev build
cargo build
# release build
cargo build --release
```

This creates a `target` directory with a directory for each build that contains among information for incremental builds a binary:

```
target
├── debug
│   ├── ...
│   └── pacman-installed-packages-explorer
└── release
    ├── ...
    └── pacman-installed-packages-explorer
```

This binary can either be invoked on its own or using:

```sh
# dev build
cargo run
# release build
cargo run --release
```

Using debug assertions this can also be identified by looking at the output:

```rust
// src/main.rs
fn main() {
    #[cfg(debug_assertions)]
    println!("debug");
    #[cfg(not(debug_assertions))]
    println!("release");
}
```

### Testing

```sh
cargo test
```

- Unit tests are usually part of the source files:

  ```rust
  // src/main.rs
  fn add(a: i32, b: i32) -> i32 {
      a + b
  }

  fn main() {
      println!("{}", add(2, 3));
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn adds_two_numbers() {
          assert_eq!(add(2, 3), 5);
      }
  }
  ```

  ```
  running 1 test
  test tests::adds_two_numbers ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```

- Integration tests can be created in a separate directory:

  ```rs
  // src/lib.rs
  pub fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  ```

  ```rs
  // tests/integration_test.rs
  use my_project::add;

  #[test]
  fn adds_two_numbers_integration() {
      assert_eq!(add(2, 3), 5);
  }
  ```

  ```
  running 1 test
  test adds_two_numbers_integration ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```

### Other

```sh
# Check if project compiles without generating a binary
cargo check
# Format code
cargo fmt
# Lint code
cargo clippy
```
