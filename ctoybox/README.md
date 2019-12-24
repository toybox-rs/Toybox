# Build instructions

The rust toybox code has moved to [toybox-rs/toybox-rs](https://github.com/toybox-rs/toybox-rs) and the pip package ``ctoybox``.

## Using OpenAI Gym baselines right now

There are several environment variables that need to be set in order to run Toybox. We keep these in the executable `start_python`. You may need to update them if your paths differ significantly from ours. 

__All examples will use `start_python`, rather than `python` or `python3`.__

In this directory, checkout the baselines repo.

    git clone https://github.com/openai/baselines.git

Follow their instructions for installing dependencies. Then, run:

    ./start_python toybox_baselines.py --alg=acer --env=toybox-breakout-v0 --num_timesteps=10000 --save_path=$PWD/breakout1e4.model

FPS is quite low on my Air because tensorflow-CPU is slow on it.
