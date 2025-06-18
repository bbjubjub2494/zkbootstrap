# ZK-Bootstrap

This is the prototype for ZK-Bootstrap, a ZK-based software distribution solution.

## Structure

```text
zkbootstrap           <-- Top-level directory, you are here
├── crates
│   ├── methods       <-- Pre-compiled zkvm guest programs (Rust)
│   ├── seeds         <-- Pre-compiled zkvm guest programs (Assembly and C)
│   └── zkbootstrap   <-- Main library
└── bins
    ├── prepare-demo  <-- Generates demo files
    └── zb            <-- Main CLI tool
```

## Simple Setup

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

then, also install Risc0 v1 via rzup:

```
curl -L https://risczero.com/install | bash
```

```
rzup install r0vm 1.2.6
```

You can install the CLI tool with

```
cargo install --path bins/zb
```

## Full Setup

The nicest way I found to obtain the stage0 compilers is through the [Nix packages collection][nixpkgs].

[Lix] is an implementation of the Nix Package Manager and Language. You can install it like so:

```
curl -sSf -L https://install.lix.systems/lix | sh -s -- install --enable-flakes
```

Then, to start a shell with access to the compilers, use

```
nix develop
```

We also need some more build tools from Risc0:

```
rzup install rust 1.81.0
rzup install cpp 2024.1.5
```

Now you can rebuild the demo files:

```
  cargo run --bin prepare-demo assets/demo1.zbb
```



[Lix]: https://lix.systems/
[nixpkgs]: https://github.com/NixOS/nixpkgs
[rust-toolchain]: rust-toolchain.toml
[rustup]: https://rustup.rs/
