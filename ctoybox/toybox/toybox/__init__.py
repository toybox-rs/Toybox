import ctoybox
from ctoybox import Toybox, Simulator, State, Input

try: 
    from gym.envs.registration import register

    # Updated to use v4 to be analogous with the    ALE versioning
    register(
        id='BreakoutToyboxNoFrameskip-v4',
        entry_point='toybox.envs.atari:BreakoutEnv',
        nondeterministic=True
    )

    register(
        id='AmidarToyboxNoFrameskip-v4',
        entry_point='toybox.envs.atari:AmidarEnv',
        nondeterministic=False
    )

    register(
        id='SpaceInvadersToyboxNoFrameskip-v4',
        entry_point='toybox.envs.atari:SpaceInvadersEnv',
        nondeterministic=False
    )

    print("Registered Toybox environments with gym.")

except:
    # ModuleNotFoundError only in 3.6 and above
    print("Loaded Toybox environments.")
