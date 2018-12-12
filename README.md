# Learning Rust

## Setup for VSCode
At the moment it seems like racer only works with the nightly builds of rust which is a big bummer so lets enable that

```
rustup install nightly
rustup default nightly
```


once that finishes install all of the dependencies for the plugin.

```
cargo install racer
cargo install rustsym
cargo install rustfmt
cargo install ripgrep
```

Then we can install the language server that vscode supports

```
rustup self update
rustup update
rustup component add rls-preview
rustup component add rust-analysis
rustup component add rust-src
```

also if you want something similar to npm's `npm install --save ...` you would need to install the `cargo-edit` plugin.

```
cargo install cargo-edit
```