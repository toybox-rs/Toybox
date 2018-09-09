# The Machine Learning Toybox

A set of games designed for causal experimentation with deep RL agents.

## Projects

- ``toybox`` - Contains core logic for games.
- ``human_play`` - Contains a front-end for playing those games as a human. 

## Mac Dev Setup Instructions
* `brew install rustup`
* `rustup-init` with the default install
* clone this repo
* `source $HOME/.cargo/env`

## Lints and Formatting in Rust

The best rust tools require the nightly compiler (because they don't want to stabilize the compiler internals yet). Follow the readme instructions to get [rustfmt](https://github.com/rust-lang-nursery/rustfmt) and [clippy](https://github.com/rust-lang-nursery/rust-clippy).

Then you can check automatically format your files with ``cargo +nightly fmt`` and peruse the best lints with ``cargo +nightly clippy``.
