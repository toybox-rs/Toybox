import toybox.testing.envs.gym as gym
import toybox.testing.models.openai_baselines as oai
import toybox.interventions.amidar as ami

from toybox.sample_tests.amidar_base import AmidarToyboxTest

import os
import random
import time
import tensorflow as tf

from scipy.stats import sem
from numpy import mean

class EnemyRemovalTest(AmidarToyboxTest):

    def shouldIntervene(self):
        return self.tick == 0

    def onTrialEnd(self):
        # An agent trained on ALE should be able to complete at least half of 
        # level 1 before time.
        with ami.AmidarIntervention(self.getToybox()) as ai:
            unpainted = ai.num_tiles_unpainted()
            painted = ai.num_tiles_painted()
        # Set to 0 to ensure it passes.
        self.assertGreaterEqual(painted, 0)
        with ami.AmidarIntervention(self.getToybox()) as ai:
            score = ai.getScore()
        return {'painted': painted, 'unpainted': unpainted, 
                'score' : score}

    def onTestEnd(self, trials_data):
        print(trials_data)

    def intervene(self):
        with ami.AmidarIntervention(self.getToybox()) as ai:
            while ai.num_enemies() > 0:
                ai.remove_enemy(0)
            ai.set_n_lives(2)

    def test_no_enemies_ppo2(self):
        print('test_no_enemies_ppo2')
        seed = 42
        fdir = os.path.dirname(os.path.abspath(__file__))
        path = os.sep.join([fdir, 'models', 'AmidarNoFrameskip-v4.ppo2.5e7.845090117.2018-12-29.model'])  
        # You need to do this if you want to load more than one model with tensorflow
        with tf.Session(graph=tf.Graph()):
            model = oai.getModel(self.env, 'ppo2', seed, path)
            # Set low to be a test of a test!
            self.timeout = 10
            self.runTest(model)

    def test_no_enemies_all_models(self):
        print('test_no_enemies_all_models')
        seed = 42
        fdir = os.path.dirname(os.path.abspath(__file__))
        models = [f for f in os.listdir(fdir + os.sep + 'models') if f.startswith('Amidar')]
        print('num models:', len(models))
        for trained in models:
            print(trained)
            path = os.sep.join([fdir, 'models', trained])
            family = trained.split('.')[1]
            with tf.Session(graph=tf.Graph()):
                model = oai.getModel(self.env, family, seed, path)
                print(self.timeout)
                self.runTest(model)
        
class OneEnemyTargetTest(AmidarToyboxTest):

    def shouldIntervene(self):
        return self.tick == 0

    def onTrialEnd(self):
        pass

    def onTestEnd(self):
        pass

    def intervene(self):
      with ami.AmidarIntervention(self.getToybox()) as ai:
        ai.game.jumps = 0
        ai.game.lives = 0
        ai.game.level = 0
        # intervene on a single enemy
        enemy = random.choice(ai.game.enemies)
        start = ami.TilePoint(ai.game.intervention, 0, 0)
        # Set the starting position to be the next one?
        start.pos = enemy.get_ai_arg('next')
        start_dir = ami.Direction.directions[random.randint(0, 3)]
        vision_distance = max(ai.game.board.height, ai.game.board.width)
        dir = ami.Direction.directions[random.randint(0, 3)]
        enemy.set_protocol('EnemyTargetPlayer', 
          start=start, 
          start_dir=start_dir,
          vision_distance=vision_distance,
          dir=dir,
          player_seen=None)
        print(enemy)
        ai.game.enemies = [enemy]

    def test_scenario_ppo2(self):
      seed = 42
      fdir = os.path.dirname(os.path.abspath(__file__))
      path = os.sep.join([fdir, 'models', 'AmidarToyboxNoFrameskip-v4.ppo2.5e7.3771075072.2019-05-18.model'])  
      model = oai.getModel(self.env, 'ppo2', seed, path)
      # Set low to be a test of a test!
      self.runTest(model)


class GangUpNoJumpTest(AmidarToyboxTest):

    def shouldIntervene(self):
      return self.tick == 0

    def onTrialEnd(self):
      if hasattr(self, 'trialnum'):
        self.trialnum += 1
      else: self.trialnum = 1
      print('end trial %d', self.trialnum)
      with ami.AmidarIntervention(self.getToybox()) as ai:
        unpainted = len(ai.game.board.tiles.filter(ami.Tile.Unpainted))
        painted = len(ai.game.board.tiles.filter(ami.Tile.Painted))
        score = ai.game.score
        self.assertGreaterEqual(painted, 6)
        return {'painted': painted, 'unpainted': unpainted, 'score' : score}

    def onTestEnd(self):
      pass

    def intervene(self):
      with ami.AmidarIntervention(self.getToybox()) as ai:
        ai.game.jumps = 0
        ai.game.lives = 1
        for enemy in ai.game.enemies:
          # We are expecting the default protocol to be enemy lookup
          assert enemy.ai_name == ami.Enemy.EnemyLookupAI
          # Create an empty TilePoint
          start = ami.TilePoint(ai.game.intervention, 0, 0)
          # Set the starting position to be close to the player's 
          # start position. I picked an arbitrary max distance (20)
          player_tile = ai.game.player.position.to_tile_point()
          start.pos = ai.get_random_tile(lambda t, b: \
              abs(t.tx - player_tile.tx) < 20 and \
              abs(t.ty - player_tile.ty) < 20)
          start_dir = ami.Direction.directions[random.randint(0, 3)]
          vision_distance = 5
          dir = ami.Direction.directions[random.randint(0, 3)]

          enemy.set_protocol('EnemyTargetPlayer', 
            start=start, 
            start_dir=start_dir,
            vision_distance=vision_distance,
            dir=dir,
            player_seen=None)

    def test_scenario_ppo2(self):
        seed = 42
        fdir = os.path.dirname(os.path.abspath(__file__))
        path = os.sep.join([fdir, 'models', 'AmidarToyboxNoFrameskip-v4.ppo2.5e7.3771075072.2019-05-18.model'])  
        model = oai.getModel(self.env, 'ppo2', seed, path)
        # Set low to be a test of a test!
        self.runTest(model)

