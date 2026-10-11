# Intro to Rust

A collection of small Rust programs and exercises. Each program is its own
binary in `src/bin/`, so they can all live in one Cargo project and be run
independently.

## Layout

```
Intro_to_Rust/
├── Cargo.toml
├── README.md
└── src/
    └── bin/
        └── second_largest_element.rs   # one file = one runnable program
```

## Running a program

The file name (without `.rs`) is the binary name:

```sh
cargo run --bin second_largest_element
```

## Testing

```sh
cargo test                                   # all programs
cargo test --bin second_largest_element      # one program
```

## Adding a new program

1. Create `src/bin/<name>.rs` with its own `fn main()`.
2. Run it with `cargo run --bin <name>`.

Minimal template:

```rust
fn main() {
    println!("Hello from <name>!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

## Programs

| Binary                   | Description                                          |
| ------------------------ | ---------------------------------------------------- |
| `second_largest_element` | Find the second largest distinct element (-1 if none) |
