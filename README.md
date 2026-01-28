# Rust Book Playground

This monorepo contains Rust projects and exercises following [The Rust Programming Language](https://doc.rust-lang.org/book/) ("the Rust Book"). Each subdirectory is a standalone Cargo project corresponding to a chapter or concept from the book.

## Structure

- `helloworld/` – First Rust program: "Hello, World!"
- `hello_cargo/` – Introduction to Cargo, Rust’s package manager
- `variables/` – Variables, mutability, and shadowing
- `functions/` – Functions, parameters, and return values
- `guessing_game/` – Complete guessing game project

## Usage

You can run any example in two ways:

**From the top-level workspace:**
```sh
cargo run -p <project>
```

**Or by navigating into a subdirectory:**
```sh
cd <project>
cargo run
```

Replace `<project>` with one of the subdirectories above.

---

Happy learning!