# Rust Lurning
It seems like Rust is a really strongly typed language, which makes me very interested in learning it.

## Setup
The development environment is not really straight forward to setup, though fortunately there is some tutorials [available](https://asquera.de/blog/2017-03-03/setting-up-a-rust-devenv/)  that being said at the very least it seems to have pretty goos vscode support.

- Install rust by doing 
```
curl https://sh.rustup.rs -sSf | sh
```

Do a bunch of other stuff.... I really should have been taking notes on this, maybe next time.

### Racer
The only way race will build is with nightly toolchain enabled.  A very important step i was missing is to set that toolchain as default by running
```
rustup override set nightly
```

## Run 
```
cargo run
```
very straight forward indeed.