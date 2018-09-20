from gym.envs.registration import register

register(
    id='toybox-breakout-v0',
    entry_point='toybox.envs.atari:BreakoutEnv',
    nondeterministic=False
)

register(
    id='toybox-amidar-v0',
    entry_point='toybox.envs.atari:AmidarEnv',
    nondeterministic=False
)
