# The Reinforcement Learning Toybox [![Build Status](https://travis-ci.com/jjfiv/toybox.svg?token=wqGZxUYsDSPaq1jz2zn6&branch=master)](https://travis-ci.com/jjfiv/toybox)

A set of games designed for testing deep RL agents.

If you use this code, or otherwise are inspired by our white-box testing approach, please cite our [NeurIPS workshop paper](https://arxiv.org/abs/1812.02850):

```
@inproceedings{foley2018toybox,
  title={{Toybox: Better Atari Environments for Testing Reinforcement Learning Agents}},
  author={Foley, John and Tosch, Emma and Clary, Kaleigh and Jensen, David},
  booktitle={{NeurIPS 2018 Workshop on Systems for ML}},
  year={2018}
}
```

## How accurate are your games?

[Watch four minutes of agents playing each game](https://www.youtube.com/watch?v=spx_YQQW1Lw). Both ALE implementations and Toybox implementations have their idiosyncracies, but the core gameplay and concepts have been captured. Pull requests always welcome to improve fidelity.

## Projects

- ``toybox`` - Contains core logic for games.
- ``ctoybox`` - Contains C API for toybox; and our python code, e.g., Gym environment bindings.

## Play the games (using pygame)

    cargo build --release
    cd ctoybox
    ./start_python human_play.py breakout
    ./start_python human_play.py amidar
    ./start_python human_play.py space_invaders

## Mac Dev Setup Instructions
* `brew install rustup`
* `rustup-init` with the default install
* clone this repo
* `source $HOME/.cargo/env`
* `cd ctoybox/toybox && python setup.py install`

## Lints and Formatting in Rust

Follow the readme instructions to get [rustfmt](https://github.com/rust-lang-nursery/rustfmt) and [clippy](https://github.com/rust-lang-nursery/rust-clippy).

Then you can check automatically format your files with ``cargo fmt`` and peruse the best lints with ``cargo clippy``.

A pre-commit hook will ensure that your code is always properly formatted. To do this, run

`git config core.hooksPath .githooks`

from the top-level directory. This will ensure that your files are formatted properly pior to committing.

## Python

Tensorflow, OpenAI Gym, OpenCV, and other libraries may or may not break with various Python versions. We have confirmed that the code in this repository will work with the following Python versions:

* 3.5


# Developing New Games

## Get starting images for reference from ALE

`./scripts/utils/start_images --help` 
