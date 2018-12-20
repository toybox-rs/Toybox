# The Machine Learning Toybox [![Build Status](https://travis-ci.com/jjfiv/toybox.svg?token=wqGZxUYsDSPaq1jz2zn6&branch=master)](https://travis-ci.com/jjfiv/toybox)

A set of games designed for causal experimentation with deep RL agents.

## Projects

- ``toybox`` - Contains core logic for games.
- ``ctoybox`` - Contains OpenAI Gym environment bindings.
- ``breakout-web-play`` - Contains a web front-end for playing those games as a human.

## Play the games (using pygame)

    cargo build --release
    cd ctoybox
    ./start_python human_play.py breakout

## Mac Dev Setup Instructions
* `brew install rustup`
* `rustup-init` with the default install
* clone this repo
* `source $HOME/.cargo/env`

## Lints and Formatting in Rust

The best rust tools require the nightly compiler (because they don't want to stabilize the compiler internals yet). Follow the readme instructions to get [rustfmt](https://github.com/rust-lang-nursery/rustfmt) and [clippy](https://github.com/rust-lang-nursery/rust-clippy).

Then you can check automatically format your files with ``cargo +nightly fmt`` and peruse the best lints with ``cargo +nightly clippy``.

A pre-commit hook will ensure that your code is always properly formatted. To do this, run

`git config core.hooksPath .githooks`

from the top-level directory. This will ensure that your files are formatted properly pior to committing.

## Python

Tensorflow, OpenAI Gym, OpenCV, and other libraries may or may not break with various Python versions. We have confirmed that the code in this repository will work with the following Python versions:

* 3.5


# Developing New Games

## Get starting images for reference from ALE

`./scripts/utils/start_images --help` 
