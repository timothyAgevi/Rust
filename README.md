# Rust

## Installation on Ubuntu
https://doc.rust-lang.org/stable/book/

comand:

# sudo apt  install curl
#  curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
# source $HOME/.cargo/env
# source $HOME/.bashrc  // This is only for bash shell users 
# source $HOME/.profile

### Verification

# rustc --version //rustc 1.62.0
# cargo--version
# rustup --version //1.24
info: This is the version for the rustup toolchain manager, not //the rustc compiler.
info: The currently active `rustc` version is `rustc 1.61.0 (fe5b13d68 2022-05-18)`

##Development Environment
-install  rust-analyzer extension and rust Extension Pack
##NB
-packages are called crates and added on Cargo.toml
-Go to Crates.io and copy rand dependency
-install nightly toolkit version:
# rustup toolchain install nightly

##comands
-cargo build
cargo run
rustup toolchain list
cargo new <appname>
# cargo expand //can install rust inaries

## Conventions
-Functions and variables written insnake case(small letter,word separeted by underscore)
-Last expresion  without semicolon in every function is automatically returned without necesairl having 'return' keyword.
-Variables must be declared as mutable inorder to be mutated

# Owenership

//1. Each value in Rust is owned by a variable

//2.When the owner goes out of scope,the value will be dealloated.

//3.there can only be 1 owner at a time.

# References nad Borrowing
-Prevents data races at compile time.
-can have many immutable borrows but only 1 mutable borrow.
-references are immutable by default.(&)