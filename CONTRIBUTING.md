# Contributing to Rusty-JavaC

Rusty-JavaC is an early-stage Java compiler written in Rust. Treat every change as compiler work: keep it small, test the generated class files, and leave the code easier to understand than you found it.

## Setup

Required tools:

- Rust 1.85+ with Cargo
- Java 21+ for `javac`, `javap`, and JVM verification
- Git

Build and run the Rust tests:

```bash
cargo build --locked
cargo test --locked
```

For bytecode, lowering, or type changes, also compile at least one Java fixture and inspect the generated class:

```bash
cargo run --locked --example compiler-example -- --output-dir target/test-output tests/java/HelloWorld.java
javap -v -c target/test-output/HelloWorld.class
java -Xverify:all -cp target/test-output HelloWorld
```

## Project Layout

The project is a single Cargo package. Compiler stages are regular Rust modules under `src/`:

```text
src/
|- ast           # Syntax node wrappers over the parsed tree
|- lexer         # Tokenization and Unicode escape handling
|- parser        # Recursive-descent parser
|- hir           # High-level IR, lowering, and expression inference
|- ty            # Java type model, descriptors, and assignability
|- call_resolver # Class catalog and method/constructor lookup
|- bytecode      # JVM bytecode generation and bytecode validation
|- classfile     # .class reading and writing helpers
|- diagnostics   # User-facing diagnostic rendering
`- compiler      # Classpath, incremental state, and compile pipeline
```

The compile path is:

```text
Java source
  -> lexer
  -> parser
  -> ast
  -> hir lowering
  -> type and call resolution
  -> bytecode generation
  -> classfile writer
```

`src/lib.rs` should stay small. Add new implementation inside the stage that owns it, then expose only the API that other stages need.

## Change Guidelines

Keep compiler behavior and structure separate:

- Parser code should describe syntax, not Java semantics.
- Lowering should translate syntax into HIR and preserve useful source spans.
- Type inference and call resolution should avoid hardcoded one-off cases when the class catalog can answer the question.
- Bytecode generation should consume typed/resolved HIR instead of re-parsing syntax decisions.
- Diagnostics should report the real source span and explain the fix when the compiler knows one.

Prefer small modules with clear ownership. If a file grows because it handles several independent concepts, split by concept rather than by arbitrary line count.

Do not add tests that only assert exact pretty-printed diagnostic text unless the output is intentionally stable. Prefer Java fixtures, JVM verification, `javap` inspection, or snapshot tests for rendered diagnostics.

## Java Fixtures

Put integration fixtures under `tests/java/`. A good fixture:

- compiles with `javac`
- exercises one compiler capability or one known edge case
- can be verified with `java -Xverify:all` when it has a runnable `main`
- avoids unrelated language features that make failures harder to diagnose

When changing bytecode output, compare against `javac` when practical. Exact bytecode does not need to match, but verifier behavior and runtime behavior should.

## Checklist Before Commit

Run the narrowest useful checks first, then the full set before committing:

```bash
cargo fmt --all
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
```

For compiler changes, also compile the affected fixtures:

```bash
cargo run --locked --example compiler-example -- --output-dir target/java-fixtures-rusty tests/java/HelloWorld.java
javap -v -c target/java-fixtures-rusty/HelloWorld.class
```

## Commits

Use Conventional Commits:

```text
type(scope): short description
```

Common types are `feat`, `fix`, `refactor`, `docs`, `test`, `ci`, and `chore`. Useful scopes are module names such as `parser`, `hir`, `bytecode`, `compiler`, `diagnostics`, and `classfile`.

Keep commits focused. If a feature needs parser, lowering, bytecode, and tests, split it into reviewable commits when possible.

## Roadmap

Use [TODO.md](TODO.md) as the working backlog. If a PR finishes an item, update the checkbox in the same PR.

## License

By contributing, you agree that your contributions are licensed under the [MIT License](LICENSE).
