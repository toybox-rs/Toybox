from gym.envs.registration import register

register(
    id='toybox-breakout-v0',
    entry_point='toybox.envs:BreakoutEnv',
)

register(
    id='toybox-amidar-v0',
    entry_point='toybox.envs:AmidarEnv'
)
