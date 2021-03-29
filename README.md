# The Reinforcement Learning Toybox ![CI](https://github.com/toybox-rs/Toybox/workflows/CI/badge.svg)

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

We have a lenghtier paper on [ArXiV](https://arxiv.org/pdf/1905.02825.pdf) and can provide a draft of a non-public paper on our acceptance testing framework by request (email at etosch at cs dot umass dot edu).

## How accurate are your games?

[Watch four minutes of agents playing each game](https://www.youtube.com/watch?v=spx_YQQW1Lw). Both ALE implementations and Toybox implementations have their idiosyncracies, but the core gameplay and concepts have been captured. Pull requests always welcome to improve fidelity.

## Where is the actual Rust code?

The rust implementations of the games have moved to a different repository: [toybox-rs/toybox-rs](https://github.com/toybox-rs/toybox-rs)

## Installation
1. Create a virtual environment using your python3 installation: `${python} -m venv .env`
   * If you are on OSX, this is likely `python3`: thus, your command will be `python3 -m venv .env`
   * If you are not sure of your version, run `python --version`
2. Activate your virtual environment: `source .env/bin/activate`
3. Install Toybox:

```    
pip install ctoybox
pip install git+https://github.com/toybox-rs/Toybox
```
4. Install requirements: run `pip install -r REQUIREMENTS.txt`
5. Install baselines: `cd baselines && python setup.py isntall && cd ..`
6. Run `python setup.py install`


## Play the games (using pygame)

    pip install ctoybox pygame
    python -m ctoybox.human_play breakout
    python -m ctoybox.human_play amidar
    python -m ctoybox.human_play space_invaders

## Run the tests

Sample behavioral tests developed with Toybox are frozen and [available here](https://github.com/toybox-rs/openai-baselines-envs). These tests are featured with an OpenAI baselines integration to facilitate off-the-shelf model training.


## Python

Tensorflow, OpenAI Gym, OpenCV, and other libraries may or may not break with various Python versions. We have confirmed that the code in this repository will work with the following Python versions:

* 3.6, 3.7

## Get starting images for reference from ALE / atari_py

`./scripts/utils/start_images --help` 
