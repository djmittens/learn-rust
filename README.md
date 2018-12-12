# Learning Rust
It seems like Rust is a really strongly typed language, which makes me very interested in learning it.

## Setup
The development environment is not really straight forward to setup, though fortunately there is some tutorials [available](https://asquera.de/blog/2017-03-03/setting-up-a-rust-devenv/)  that being said at the very least it seems to have pretty goos vscode support.

- Install rust by doing 
```
curl https://sh.rustup.rs -sSf | sh
```

Do a bunCH Of other stuff.... I really should have been taking notes on this, maybe next time.

## Setup for VSCode
At the moment it seems like racer only works with the nighTLY builds of rust which is a big bummer so lets enable that

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


## The Rust Programming Language
I chose to learn from the more wordy book on the rust-lang.org site.

Thats why here ill be following,

[The Rust Programming Language](https://doc.rust-lang.org/book) Book

### Hello Cargo!

#### Creating a new project
Can be done by using the following command.
```
cargo init
```

#### Check for compiler errors.
Its sometimes useful to check for compilation errors without building, and you can do that with

```
cargo check
```

#### Releasing the code
in order to build an optimized version of the executable ready for release please use 
```
cargo build --release
```

### Guessing game

This program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high.  If the guess is correct, the game will print a congratulatory message and exit.