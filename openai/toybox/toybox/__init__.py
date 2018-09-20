from gym.envs.registration import register

register(
    id='BreakoutToyboxNoFrameskip-v0',
    entry_point='toybox.envs.atari:BreakoutEnv',
    nondeterministic=False
)

register(
    id='AmidarToyboxNoFrameskip-v0',
    entry_point='toybox.envs.atari:AmidarEnv',
    nondeterministic=False
)
