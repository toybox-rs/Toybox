from importlib import import_module

# copied from baselines.run
def _get_alg_module(alg, submodule=None):
    submodule = submodule or alg
    alg_module = import_module('.'.join(['baselines', alg, submodule]))

    return alg_module

def _get_learn_function(alg):
    # copied verbatim from baselines
    return _get_alg_module(alg).learn

def _get_learn_function_defaults(alg, env_type):
    # Copied verbaim from baselines
    try:
        alg_defaults = _get_alg_module(alg, 'defaults')
        kwargs = getattr(alg_defaults, env_type)()
    except (ImportError, AttributeError):
        kwargs = {}
    return kwargs


def getModel(env, family, seed, model_path):
  learn = _get_learn_function(family)
  alg_kwargs = _get_learn_function_defaults(family, 'atari')
  alg_kwargs['network'] = 'cnn'
  alg_kwargs['load_path'] = model_path
  return learn(env=env, seed=seed, total_timesteps=0, **alg_kwargs)

def takeAction(self, model):
  self.action = model.step(self.obs)[0]  