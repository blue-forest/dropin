# drop'in Compiler

[![License](https://img.shields.io/github/license/blue-forest/dropin)](../COPYING)
[![Maintenance](https://img.shields.io/badge/maintained-no-red.svg)](#)
[![Stability](https://img.shields.io/badge/stable-no-red.svg)](#)
[![Crates.io latest version](https://img.shields.io/crates/v/dropin)](https://crates.io/crates/dropin/versions)
[![Crates.io total downloads](https://img.shields.io/crates/d/dropin)](https://crates.io/crates/dropin)

This directory contains the source code of the drop'in compiler written in Rust and that compiles the [drop'in recipes](https://dropin.recipes) into WebAssembly files.

This source code is in an experimental state and will be fully documented in its final version, we are NOT currently working on it and the code is here for testing purposes only.

Warning : **we do not recommend to use this code in a production environment**, even if it theoretically works we haven't finished writing tests to make sure everything works and avoid regressions.

You are free to explore the code and fork it, we are open to [Issues and Pull Requests](https://github.com/blue-forest/contributing).

The codebase is [licensed under GNU Affero General Public License v3](../COPYING), you have the right to exploit this source code but it must remain open-source and under the same license, thanks for your support !

If you want to join the development and contribute to the project, please reach us at dropin@blueforest.cc.


## Documentation
You can find the documentation of the drop'in language on [dropin.recipes](https://dropin.recipes).

You can find more informations in [`./etc/notes`](./etc/notes) about :
 - [the CLI](./etc/notes/CLI.md)
 - [the File Structure](./etc/notes/Structure.md)
 - [the Development](./etc/notes/Development.md)
 - [the Memory management](./etc/notes/memory.md)


## Contributors
- Nazim Lachter ([@n4zim](https://github.com/n4zim))
- Vulcain ([@vulc41n](https://github.com/vulc41n))
