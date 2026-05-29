# TODO

This file tracks known missing or incomplete compiler work. Check an item only when the implementation is covered by fixtures or verifier checks where applicable.

## Already Present

- [x] Single-package Rust project layout under `src/`
- [x] Lexer, parser, HIR lowering, bytecode generation, and classfile writing pipeline
- [x] Cargo-style diagnostics for parser/lowering/bytecode validation errors
- [x] Package output paths
- [x] Classpath scanning for directories, jars, class files, and Java source files
- [x] Platform signatures for common `java.lang`, `java.util`, `java.io`, `java.net`, `java.nio.file`, `java.time`, and `java.util.function` classes
- [x] Basic expression inference shared by lowering, validation, and bytecode generation
- [x] Lambda lowering through `invokedynamic` and `LambdaMetafactory`
- [x] Anonymous class classfile generation for current fixtures
- [x] SourceFile, LineNumberTable, LocalVariableTable, StackMapTable, and generic Signature coverage for supported fixtures
- [x] Assignment expression/effect codegen modes that avoid unnecessary `dup`/`pop` for supported assignments
- [x] Incremental compile state scaffold
- [x] CI path filters that skip docs-only changes unless manually triggered

## Parser

- [ ] Parse annotations and preserve them in syntax/HIR
- [ ] Parse annotation declarations
- [ ] Parse interfaces completely, including default and static methods
- [ ] Parse enum declarations, enum constants, and enum constant bodies
- [ ] Parse records and compact constructors
- [ ] Parse sealed, non-sealed, and permits clauses
- [ ] Parse module-info.java
- [ ] Parse try/catch/finally
- [ ] Parse try-with-resources
- [ ] Parse synchronized statements and methods
- [ ] Parse assert statements
- [ ] Parse throw statements
- [ ] Parse enhanced for loops robustly
- [ ] Parse labeled statements and labeled break/continue
- [ ] Parse local classes in method bodies
- [ ] Parse nested static and non-static member classes
- [ ] Parse wildcard generic types with `extends` and `super`
- [ ] Parse generic methods and constructors
- [ ] Parse varargs declarations
- [ ] Parse method references such as `String::valueOf`
- [ ] Parse array initializer edge cases and nested initializers
- [ ] Add parser recovery that keeps useful later errors

## HIR And Lowering

- [ ] Split `src/hir/lowering/expr.rs` by expression family
- [ ] Split `src/hir/lowering/stmt.rs` by statement family
- [ ] Move binding decisions out of syntax-shaped lowering code
- [ ] Preserve precise source spans for every lowered expression and statement
- [ ] Lower try/catch/finally into explicit HIR
- [ ] Lower try-with-resources with correct close ordering and suppressed exceptions
- [ ] Lower throw statements
- [ ] Lower synchronized blocks and methods
- [ ] Lower enum declarations into JVM-compatible classes
- [ ] Lower records into fields, accessors, constructor, and standard methods
- [ ] Lower local classes
- [ ] Lower nested member classes
- [ ] Lower captured locals for lambdas and inner classes
- [ ] Lower method references
- [ ] Lower enhanced for loops over arrays
- [ ] Lower enhanced for loops over `Iterable`
- [ ] Track pattern variable scope through boolean control flow
- [ ] Track definite assignment in HIR instead of late bytecode checks
- [ ] Represent typed HIR expressions after inference/resolution

## Type System

- [ ] Implement generic type substitution for fields and methods
- [ ] Implement generic method inference
- [ ] Implement wildcard capture conversion
- [ ] Implement full numeric promotion and constant narrowing rules
- [ ] Implement boxing and unboxing conversions consistently
- [ ] Implement varargs applicability
- [ ] Implement overload resolution phases matching Java rules
- [ ] Resolve inherited methods through full superclass and interface chains
- [ ] Resolve default interface methods
- [ ] Resolve bridge methods from classfile metadata when needed
- [ ] Track access checks for public/protected/private/package-private members
- [ ] Track static-vs-instance misuse as semantic errors
- [ ] Validate checked exceptions
- [ ] Validate unreachable statements
- [ ] Validate definite return for non-void methods
- [ ] Validate final fields and final locals
- [ ] Validate sealed class inheritance rules

## Classpath And Platform

- [ ] Store field signatures from classpath `.class` files
- [ ] Store method generic signatures from classpath `.class` files
- [ ] Store superclass and interface relationships from classpath `.class` files
- [ ] Store access flags for classpath classes, fields, and methods
- [ ] Resolve nested classes from InnerClasses attributes
- [ ] Resolve imports to source files and compile dependency sources in order
- [ ] Detect duplicate classes across source and classpath entries
- [ ] Report missing imports with the import span, not the next valid import line
- [ ] Support sourcepath separately from classpath
- [ ] Cache classpath scans across incremental compiles
- [ ] Expand platform signatures without turning them into ad hoc call hardcoding
- [ ] Add platform coverage for common collection, stream, regex, and concurrency APIs

## Bytecode Generation

- [ ] Emit exception tables for try/catch/finally
- [ ] Emit monitorenter/monitorexit for synchronized code
- [ ] Verify StackMapTable generation for all loop and branch shapes
- [ ] Verify LineNumberTable output for multi-line expressions
- [ ] Verify LocalVariableTable slot lifetimes for shadowing and nested scopes
- [ ] Generate InnerClasses attributes for member, local, and anonymous classes
- [ ] Generate EnclosingMethod attributes for local and anonymous classes
- [ ] Generate complete NestHost and NestMembers metadata
- [ ] Generate bootstrap metadata for method references
- [ ] Generate efficient string switch bytecode
- [ ] Generate enum switch mapping compatible with Java semantics
- [ ] Remove temporary locals introduced only for codegen convenience
- [ ] Replace reflection-style array construction with direct Java array bytecode when possible
- [ ] Compare generated bytecode against `javac` for representative fixtures
- [ ] Add verifier tests for every fixture with a runnable `main`

## Diagnostics

- [ ] Give every parse error a stable diagnostic code
- [ ] Give every lowering error a precise source span
- [ ] Give every semantic error a precise source span
- [ ] Report multiple independent errors in one compile
- [ ] Add suggestions for common import typos
- [ ] Add suggestions for missing classpath entries
- [ ] Add suggestions for method overload mismatches
- [ ] Distinguish syntax errors from unsupported language features
- [ ] Use snapshot tests for rendered diagnostics
- [ ] Keep diagnostic renderer separate from compiler logic

## Tests And Fixtures

- [ ] Add fixture runner that compiles all `tests/java/*.java`
- [ ] Add `javap -v` smoke checks in a reusable test helper
- [ ] Add JVM verifier checks in a reusable test helper
- [ ] Add `javac` comparison tests for accepted fixtures
- [ ] Add negative compile fixtures for parser errors
- [ ] Add negative compile fixtures for missing symbols
- [ ] Add negative compile fixtures for type mismatches
- [ ] Add snapshot tests for diagnostics
- [ ] Add real-world classpath fixture using ASM
- [ ] Add fixtures for Java collections, streams, regex, time, file IO, and networking
- [ ] Add fixtures for packages with cross-file references
- [ ] Add fixtures for nested classes and local classes
- [ ] Add fixtures for exception handling
- [ ] Add fuzz tests for lexer/parser stability

## Public API And Tooling

- [ ] Document the library API with rustdoc examples
- [ ] Expose a structured compile result instead of only rendered errors
- [ ] Expose parsed syntax for tools that do not need bytecode
- [ ] Expose HIR for analysis tools
- [ ] Add a stable API for custom classpath providers
- [ ] Add a stable API for incremental compilation state
- [ ] Add benchmark coverage against `javac` on small and medium inputs
- [ ] Add a release checklist for crates.io publishing
- [ ] Keep the package published as one `rusty-javac` crate
