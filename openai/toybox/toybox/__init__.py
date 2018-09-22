from gym.envs.registration import register
import toybox.toybox as toybox
import toybox.envs as envs
import toybox.interventions as interventions
import toybox.sample_tests as sample_tests

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

print("Loaded Toybox environments.")