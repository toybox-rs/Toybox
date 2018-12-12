from gym.envs.registration import register
import toybox.toybox as toybox
import toybox.envs as envs
import toybox.interventions as interventions
import toybox.sample_tests as sample_tests

# Updated to use v4 to be analogous with the ALE versioning
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

print("Loaded Toybox environments.")