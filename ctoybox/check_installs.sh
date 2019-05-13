#!/bin/bash
#pip3 install gym[atari] --user
#pip3 install 'tensorboard<1.8.0,>=1.7.0' --user
curl https://sh.rustup.rs -sSf > install_rust.sh
chmod +x install_rust.sh
./install_rust.sh -y
source $HOME/.cargo/env
rustup default stable
cargo build --release
