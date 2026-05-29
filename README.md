# Rusty-JavaC

> [!WARNING]
> This project is still under heavy development. It is not intended to replace the official Java compiler yet.

Rusty-JavaC is a Java compiler written in Rust. It parses Java source, lowers it into an internal representation, resolves types and calls, and writes JVM `.class` files.

## Why

The official Java compiler is written in Java. Rusty-JavaC explores a Rust implementation with:

- explicit ownership and error handling inside the compiler
- Cargo-based distribution for Rust tooling
- a reusable library API for Java analysis and bytecode-producing tools
- room for experiments around diagnostics, incremental compilation, and classpath analysis

## Pipeline

The compiler is one Cargo package with stages split into modules under `src/`:

```text
Java source
  -> lexer
  -> parser
  -> ast
  -> hir lowering
  -> type and call resolution
  -> bytecode generation
  -> JVM .class file
```

## Use It

As a library:

```bash
cargo add rusty-javac
```

Or in `Cargo.toml`:

```toml
rusty-javac = "VERSION"
```

As the example compiler:

```bash
cargo run --example compiler-example -- --output-dir target/classes tests/java/HelloWorld.java
```

## Build

Requirements:

- Rust 1.85+
- Java 21+ for fixture comparison, `javap`, and JVM verification

```bash
git clone https://github.com/Eatgrapes/Rusty-JavaC.git
cd Rusty-JavaC
cargo build --locked
cargo test --locked
```

## Status

The compiler already handles a growing subset of Java syntax, bytecode generation, classpath/source lookup, package output, lambda lowering, anonymous classes, and platform class signatures.

Large parts of Java are still incomplete. See [TODO.md](TODO.md) for the current backlog.

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a PR. Good contributions usually include a focused Java fixture, verifier coverage when applicable, and a commit message following Conventional Commits.

## License

[MIT](LICENSE)
