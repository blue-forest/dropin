# Project file structure

## Root package: CLI

As you can see in [`Cargo.toml`](../Cargo.toml), the root package is an
interactive CLI to help you use drop'in.

[You can learn more about this CLI here](CLI.md)

It is also a rust workspace defining the following crates:
- dropin-bootstrap
- dropin-debugger
- dropin-utils

## Bootstrap

As the drop'in compiler is written in the drop'in language, we face the
chicken-egg problem: to get a compiler, we need a compiler.

[Bootstrapping](https://en.wikipedia.org/wiki/Bootstrapping_(compilers)) is a
technique to answer this problem. The crate 
[`dropin-bootstrap`](../dropin-bootstrap) is a minimal drop'in compiler written
in Rust. It compiles the feature-rich compiler, which will be distributed as
the official drop'in compiler.

## Debugger

Some low level features can be tough to debug. The crate
[`dropin-debugger`](../dropin-debugger) provides tools to facilitate WebAssembly
modules debugging.

## Utils

Some small functions are needed in several crates. The crate
[`dropin-utils`](../dropin-utils) contains some dependency-free functions that
power the other packages.

## Sandbox

The folder [`etc/sandbox`](../etc/sandbox) is a WebAssembly playground where
experiments are made to explore possible solutions for a given problem.
