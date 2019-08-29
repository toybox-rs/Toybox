import toybox_envs.atari

try: 
    from gym.envs.registration import register

    # Updated to use v4 to be analogous with the    ALE versioning
    register(
        id='BreakoutToyboxNoFrameskip-v4',
        entry_point='toybox_envs.atari:BreakoutEnv',
        nondeterministic=True
    )

    register(
        id='AmidarToyboxNoFrameskip-v4',
        entry_point='toybox_envs.atari:AmidarEnv',
        nondeterministic=False
    )

    register(
        id='SpaceInvadersToyboxNoFrameskip-v4',
        entry_point='toybox_envs.atari:SpaceInvadersEnv',
        nondeterministic=False
    )

    print("Registered Toybox environments with gym.")

except:
    # ModuleNotFoundError only in 3.6 and above
    print("Loaded Toybox environments.")
