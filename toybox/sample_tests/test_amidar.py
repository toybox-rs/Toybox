import toybox.testing.envs.gym as gym
import toybox.interventions.amidar as ami
#import toybox.testing.behavior as behavior
import toybox.testing.models.openai_baselines as oai
import os
import random
import time
import tensorflow as tf

from scipy.stats import sem
from numpy import mean
from toybox.sample_tests.base import AmidarToyboxTestBase


def generate_random_dir(intervention):
    return ami.Direction(intervention, ami.Direction.directions[random.randint(0, 3)])


class EnemyRemovalTest(AmidarToyboxTestBase):

    def setUp(self): super().setUp(trials=5, timeout=500)

    def shouldIntervene(self, obj=None): return self.tick == 0

    def onTrialEnd(self):
      self.assertIsNotNone(self.final_state)
      game = ami.Amidar.decode(self, self.final_state, ami.Amidar)
      painted = sum([
          sum([int(tile.tag == ami.Tile.Painted) for tile in row])\
               for row in game.board.tiles]) 
      print('painted:', painted, 'score', game.score)
      self.assertGreaterEqual(painted, 10)
      return {'painted': painted, 'score' : game.score}

    def onTestEnd(self): pass

    def intervene(self, obj=None):
      with ami.AmidarIntervention(self.getToybox()) as intervention:
        intervention.game.lives = 1
        intervention.game.enemies.clear()

    def test_no_enemies_ppo2(self):
        print('testing test_no_enemies_ppo2')
        seed = 42
        path = 'models/AmidarToyboxNoFrameskip-v4.regress.model'
        # # You need to do this if you want to load more than one model with tensorflow
        with tf.Session(graph=tf.Graph()):
            model = oai.getModel(self.env, 'ppo2', seed, path)
            self.runTest(model)

    def test_no_enemies_all_models(self):
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
                self.runTest(model)
        
class OneEnemyTargetTest(AmidarToyboxTestBase):

    def setUp(self): super().setUp(trials=5, timeout=500)

    def shouldIntervene(self, obj=None): return self.tick == 0

    def onTrialEnd(self):
      self.assertIsNotNone(self.final_state)
      game = ami.Amidar.decode(self, self.final_state, ami.Amidar)
      painted = sum([
          sum([int(tile.tag == ami.Tile.Painted) for tile in row])\
               for row in game.board.tiles]) 
      print('painted:', painted, 'score', game.score)
      self.assertGreaterEqual(painted, 10)
      return {'painted': painted, 'score' : game.score}

    def onTestEnd(self): pass

    def intervene(self, obj=None):
      with ami.AmidarIntervention(self.getToybox()) as intervention:
        game = intervention.game
        game.lives = 1
        game.jumps = 0
        # intervene on a single enemy
        enemy = random.choice(game.enemies)
        start = ami.TilePoint(game.intervention, 0, 0)
        # Set the starting position to be the next one?
        start_dir = generate_random_dir(intervention)
        vision_distance = max(game.board.height, game.board.width)
        dir = generate_random_dir(intervention)
        intervention.set_enemy_protocol(enemy, 'EnemyTargetPlayer', 
          start=start, 
          start_dir=start_dir,
          vision_distance=vision_distance,
          dir=dir,
          player_seen=None)
        game.enemies.clear()
        game.enemies.append(enemy)
        self.assertTrue(intervention.dirty_state)

    def test_scenario_ppo2(self):
      seed = 42
    #   fdir = os.path.dirname(os.path.abspath(__file__))
    #   path = os.sep.join([fdir, 'models', 'AmidarToyboxNoFrameskip-v4.ppo2.5e7.3771075072.2019-05-18.model'])
      path = 'models/AmidarToyboxNoFrameskip-v4.regress.model'
      with tf.Session(graph=tf.Graph()):
        model = oai.getModel(self.env, 'ppo2', seed, path)
        # Set low to be a test of a test!
        self.runTest(model)

class GangUpNoJumpRandomTest(AmidarToyboxTestBase):

    def setUp(self): super().setUp(trials=5, timeout=500)

    def shouldIntervene(self, obj=None): return self.tick == 0

    def onTrialEnd(self):
      self.assertIsNotNone(self.final_state)
      game = ami.Amidar.decode(self, self.final_state, ami.Amidar)
      painted = sum([
          sum([int(tile.tag == ami.Tile.Painted) for tile in row])\
               for row in game.board.tiles]) 
      print('painted:', painted, 'score', game.score)
      self.assertGreaterEqual(painted, 10)
      return {'painted': painted, 'score' : game.score}


    def onTestEnd(self): pass

    def intervene(self, obj=None):
      with ami.AmidarIntervention(self.getToybox()) as intervention:
        game = intervention.game
        game.jumps = 0
        game.lives = 1
        num_enemies = len(game.enemies)

        sample_enemy = game.enemies[0] 
        game.enemies.clear()

        player_pos = intervention.worldpoint_to_tilepoint(game.player.position)

        min_distance = 2

        def is_min_distance(t, e):
            t = intervention.tile_to_tilepoint(t)
            e = intervention.worldpoint_to_tilepoint(e.position)
            return abs(t.tx - player_pos.tx) > 2 and \
                   abs(t.ty - player_pos.ty) > 2 and \
                   abs(t.tx - e.tx) > 2 and \
                   abs(t.ty - e.ty) > 2

        while num_enemies > 0:
          print('num_enemies:', num_enemies)
          num_enemies -= 1
          start_tile = intervention.get_random_tile(lambda t: all([is_min_distance(t, e) for e in game.enemies]))
          start = intervention.tile_to_tilepoint(start_tile)
          # Set the starting position to be close to the player's 
          # start position. I picked an arbitrary max distance (20)
          start_dir = generate_random_dir(intervention)
          print('random start:', start, start_dir)
          dir = generate_random_dir(intervention)

          # Create a copy.
          enemy = ami.Enemy.decode(intervention, sample_enemy.encode(), ami.Enemy)
          intervention.set_enemy_protocol(enemy, ami.MovementAI.EnemyRandomMvmt, 
            start=start, 
            start_dir=start_dir,
            dir=dir)
          game.enemies.append(enemy)

    def test_scenario_ppo2(self):
      seed = 42
    #   fdir = os.path.dirname(os.path.abspath(__file__))
    #   path = os.sep.join([fdir, 'models',  'AmidarToyboxNoFrameskip-v4.ppo2.5e7.3771075072.2019-05-18.model'])  
      path = 'models/AmidarToyboxNoFrameskip-v4.regress.model'
      model = oai.getModel(self.env, 'ppo2', seed, path)
      # Set low to be a test of a test!
      self.runTest(model)

class GangUpNoJumpTargetTest(AmidarToyboxTestBase):

    def setUp(self): super().setUp(trials=5, timeout=500)
  
    def shouldIntervene(self, obj=None): return self.tick == 0

    def onTrialEnd(self):
      self.assertIsNotNone(self.final_state)
      game = ami.Amidar.decode(self, self.final_state, ami.Amidar)
      painted = sum([
          sum([int(tile.tag == ami.Tile.Painted) for tile in row])\
               for row in game.board.tiles]) 
      print('painted:', painted, 'score', game.score)
      self.assertGreaterEqual(painted, 10)
      return {'painted': painted, 'score' : game.score}


    def onTestEnd(self):
      pass

    def intervene(self, obj=None):
      with ami.AmidarIntervention(self.getToybox()) as intervention:
        game = intervention.game
        game.jumps = 0
        game.lives = 1
        for enemy in game.enemies:
          # We are expecting the default protocol to be enemy lookup
          assert enemy.ai.protocol == ami.MovementAI.EnemyLookupAI
          # Set the starting position to be close to the player's 
          # start position. I picked an arbitrary max distance (20)
          player_tile = intervention.worldpoint_to_tilepoint(game.player.position)
          start_tile = intervention.get_random_tile(lambda t: \
              abs(intervention.tile_to_tilepoint(t).tx - player_tile.tx) < 20 and \
              abs(intervention.tile_to_tilepoint(t).ty - player_tile.ty) < 20)
          start = intervention.tile_to_tilepoint(start_tile)
          start_dir = generate_random_dir(intervention)
          vision_distance = 5
          dir = generate_random_dir(intervention)

          intervention.set_enemy_protocol(enemy, 'EnemyTargetPlayer',
            start=start, 
            start_dir=start_dir,
            vision_distance=vision_distance,
            dir=dir,
            player_seen=None)

    def test_scenario_ppo2(self):
      seed = 42
      fdir = os.path.dirname(os.path.abspath(__file__))
    #   path = os.sep.join([fdir, 'models',  'AmidarToyboxNoFrameskip-v4.ppo2.5e7.3771075072.2019-05-18.model'])  
      path = 'models/AmidarToyboxNoFrameskip-v4.regress.model'
      model = oai.getModel(self.env, 'ppo2', seed, path)
      # Set low to be a test of a test!
      self.runTest(model)