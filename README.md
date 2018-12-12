# Learning Rust
It seems like Rust is a really strongly typed language, which makes me very interested in learning it.

## Setup
The development environment is not really straight forward to setup, though fortunately there is some tutorials [available](https://asquera.de/blog/2017-03-03/setting-up-a-rust-devenv/)  that being said at the very least it seems to have pretty goos vscode support.

- Install rust by doing 
```
curl https://sh.rustup.rs -sSf | sh
```

Do a bunch of other stuff.... I really should have been taking notes on this, maybe next time.

## Setup for VSCode
At the moment it seems like racer only works with the nightly builds of rust which is a big bummer so lets enable that

```
rustup install nightly
rustup default nightly
```

```
rustup override set nightly
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