import toybox.testing.envs.gym as gym
from gym import logger
import toybox.interventions.amidar as ami
import toybox.testing.models.openai_baselines as oai
import os
import random
import math
import numpy as np
import time
import tensorflow as tf

from resources.lookup_util import shift_enemy_defaults
from toybox.sample_tests.base import AmidarToyboxTestBase
from toybox.sample_tests.test_amidar import generate_random_dir

class AmidarToyboxTestExp(AmidarToyboxTestBase):

    def setUp(self, trials=10, timeout=500, *args, **kwargs):
        super().setUp(trials=trials, timeout=timeout, *args, **kwargs)
        self.exp_keys = []
        # remove prior results file
        self.resultsfile = os.path.join('results','amidar_shift.txt')
        if os.path.exists(self.resultsfile):
            os.remove(self.resultsfile)

    def resetEnv(self):
      super().resetEnv()

    def onTrialStart(self):
      with ami.AmidarIntervention(self.getToybox()) as intervention:
          # safe amidar: experiment mode
          intervention.game.lives = 1
          # no chase mode
          intervention.game.jump_timer = 0
          intervention.game.jumps = 0

          # no segments filled
          unpaint_these = intervention.filter_tiles(lambda t: t.tag == ami.Tile.Painted)
          for tile in unpaint_these:
              tile.tag = ami.Tile.Unpainted

    def shouldIntervene(self, obj=None): return self.tick == 0

    def onTrialEnd(self):
      self.assertIsNotNone(self.final_state)
      with ami.AmidarIntervention(self.getToybox()) as intervention:
        game = ami.Amidar.decode(intervention, self.final_state, ami.Amidar)
      painted = sum([
          sum([int(tile.tag == ami.Tile.Painted) for tile in row])\
               for row in game.board.tiles])
      print('painted:', painted, 'score', game.score)
    # store stats
      return {'painted': painted, 'score' : game.score, 'time' : self.tick,
              'ntrials' : self.trials, 'timeout' : self.timeout}

    def onTestEnd(self, trials_data):
        from scipy.stats import sem
        from numpy import mean

        print(trials_data)
        # write and print aggregate stats
        fname = self.resultsfile
        with open(fname, 'a') as f:
            info = ['model', 'score', 'painted', 'time', 'ntrials', 'timeout'] + list(self.exp_keys)
            print(' '.join(info))
            if os.path.getsize(fname) == 0:
                # if file not exist, first run
                f.write(' '.join(info)+'\n')

            # write raw
            scores = [s['score'] for s in trials_data]
            times = [t['time'] for t in trials_data]
            counts = [c['painted'] for c in trials_data]

            #  calculate aggregates
            score_mean = mean(scores)
            time_mean = mean(times)
            count_mean = mean(counts)

            # write to file
            model_name = os.path.basename(self.model_path)
            info = [model_name, str(score_mean), str(count_mean), str(time_mean), str(self.trials), str(self.timeout)]
            # store any extra args assigned
            info = info + [str(self.__dict__[v]) for v in self.exp_keys]
            # n_shift: number enemies shifted
            # t: offset sampled from uniform
            # epsilon: gaussian noise sigma
            print(' '.join(info))
            # try with lock
            # don't write over, write to file
            f.write(' '.join(info)+'\n')

    def intervene(self, obj=None): pass

    def test_shift_ppo2(self):
        return
        print('testing enemy_shift_ppo2')
        seed = 42
        path = 'models/AmidarToyboxNoFrameskip-v4.regress.model'
        # # You need to do this if you want to load more than one model with tensorflow
        with tf.compat.v1.Session(graph=tf.Graph()):
            model = oai.getModel(self.env, 'ppo2', seed, path)
            self.runTest(model)

    def test_shift_all_models(self):
        seed = 42
        print(os.getcwd())
        fdir = os.path.join(os.getcwd(), 'ijcai20')
        print('testing enemy_shift for all models in', fdir)
        #fdir = os.path.realpath(fdir) # resolve symlinks
        #print('testing enemy_shift for all models in', fdir)

        models = [f for f in os.listdir(fdir) if f.startswith('Amidar')]
        print('num models:', len(models))
        for trained in models:
            print(trained)
            path = os.sep.join([fdir, trained])
            path = os.path.realpath(path)
            family = trained.split('.')[1]
            if family == 'ppo2':
                # acktr not supported
                # a2c?
                with tf.compat.v1.Session(graph=tf.Graph()):
                    model = oai.getModel(self.env, family, seed, path)
                    self.model_path = path
                    self.runTest(model)
                    time.sleep(1)
                    #tf.keras.backend.clear_session()

class EnemyShiftTest(AmidarToyboxTestExp):

    def setUp(self, trials=30, timeout=500, *args, **kwargs):
        super().setUp(trials=trials, timeout=timeout, *args, **kwargs)

        # kwargs: default exp values
        # n_shift: number of enemies to shift
        # t: tile range to shift enemies on their track
        # epsilon: Gaussian noise to add to each enemy's shift
        self.n_shift = 5
        self.t = 20
        self.epsilon = 5

        # replace default exp values with kwargs
        # check kwarg validity
        self.exp_keys =  {'n_shift', 't', 'epsilon'}
        valid_keys = [k for k in kwargs if k in self.exp_keys]
        # complain if exp_keys contains invalid parameters
        invalid_params = set(kwargs.keys()) - set(valid_keys)
        logger.warn("Attempt to set invalid parameters: " + str(invalid_params))
        # replace default values with those set in kwargs
        self.__dict__.update((k,v) for k, v in kwargs if k in valid_keys)

        # load the EnemyLookupAI default tracks
        # each row is a tile order for an EnemyLookupAI protocol
        with open("resources/amidar_enemy_positions") as f:
            tile_lists = f.readlines()
            self.lookup_table = [e.split(' ') for e in tile_lists]

    def shouldIntervene(self, obj=None): return self.tick == 5

    def onTrialEnd(self):
      return super().onTrialEnd()

    def intervene(self, obj=None):
      # set the enemies offset on their track
      with ami.AmidarIntervention(self.getToybox()) as intervention:
        # sample enemies to shift
        assert self.n_shift <= len(intervention.game.enemies)
        move_es = random.sample(list(intervention.game.enemies), self.n_shift)
        # sample amount translated
        shf = int(random.choice(np.arange(-1*self.t,self.t)))
        shf_vec = [shf if e in move_es else 0 for e in intervention.game.enemies]
        # amount separation
        noise_vec = [int(random.choice(np.arange(-1 * self.epsilon, self.epsilon))) for _ in intervention.game.enemies]
        # assign shift amount
        shift_vec = [x+ noise_vec[i] for i, x in enumerate(shf_vec)]
        print("shifting by", shift_vec)

        # set the enemy position to the offset lookup tile position
        shift_enemy_defaults(intervention, shift_vec, self.lookup_table)

    class EnemyMvmtSwapTest(AmidarToyboxTestExp):

        def shouldIntervene(self, obj=None): return self.tick == 5

        def onTrialEnd(self):
            return super().onTrialEnd()

        def intervene(self, obj=None):
            # change enemy protocol
            with ami.AmidarIntervention(self.getToybox()) as intervention:
                for e in intervention.game.enemies:
                    print("current protocol", e.ai)
                    pass
                    new_protocol = random.choice(ami.MovementAI.mvmt_protocols)
                    protocol_kwargs = []
                    ai = intervention.set_enemy_protocol(e, new_protocol, **protocol_kwargs)
                    e.ai = ai
                    print("new protocol", e.ai)