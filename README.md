# The Machine Learning Toybox [![Build Status](https://travis-ci.com/jjfiv/toybox.svg?token=wqGZxUYsDSPaq1jz2zn6&branch=master)](https://travis-ci.com/jjfiv/toybox)

A set of games designed for causal experimentation with deep RL agents.

## Target Rust Version

For building on older GPU clusters, we target rustc 1.28:
```bash
rustup override set 1.28.0
```

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

## Python

Tensorflow, OpenAI Gym, OpenCV, and other libraries may or may not break with various Python versions. We have confirmed that the code in this repository will work with the following Python versions:

* 3.5
