from toybox.sample_tests.base import SpaceInvadersToyboxTestBase
import toybox.interventions.space_invaders as si
import toybox.testing.models.openai_baselines as oai

from collections import Counter
from numpy import random

seed = 1984
path = 'models/SpaceInvadersToyboxNoFrameskip-v4.regress.model'

class NoShields(SpaceInvadersToyboxTestBase):

    def shouldIntervene(self, obj=None): return self.tick == 0

    def onTrialEnd(self): 
      print('Survival Time: %d' % self.tick)
      return self.tick

    def onTestEnd(self): pass

    def intervene(self, obj=None):
      with si.SpaceInvadersIntervention(self.getToybox()) as intervention:
        intervention.game.shields.clear()

    def test_no_shields_ppo2(self):
      model = oai.getModel(self.env, 'ppo2', seed, path)
      self.runTest(model)

class JitterVary(SpaceInvadersToyboxTestBase):

    def shouldIntervene(self, obj=None): return self.tick == 0

    def onTrialEnd(self): 
      print('Survival Time: %d' % self.tick)
      return self.tick

    def onTestEnd(self): pass

    def intervene(self, obj=None):
      p = obj
      with si.SpaceInvadersIntervention(self.getToybox()) as intervention:
        intervention.set_jitter(p)
    
    def test_no_shields_ppo2(self):
      model = oai.getModel(self.env, 'ppo2', seed, path)
      ps = random.choice([n/10 for n in range(11)], 2)
      self.runTest(model, collection=ps)

class ShieldXs(SpaceInvadersToyboxTestBase):

    def onTrialEnd(self):
      observed = self.xs_observed
      print(observed.items())
      self.xs_observed = Counter()
      return observed

    def onTestEnd(self): pass

    def shouldIntervene(self, obj=None):
      if self.tick == 0:
        return True
      else:
        self.xs_observed[self.getToybox().query_state_json('ship_x')] += 1
        return False

    def intervene(self, obj=None):
      s1, s2, s3 = obj
      with si.SpaceInvadersIntervention(self.getToybox()) as intervention:
        game = intervention.game
        to_keep = []
        if s1:
          to_keep.append(game.shields[0])
        if s2:
          to_keep.append(game.shields[1])
        if s3:
          to_keep.append(game.shields[2])
        game.shields.clear()
        game.shields.extend(to_keep)

    def test_shieldxs_ppo2(self):
      model = oai.getModel(self.env, 'ppo2', seed, path)
      shield_configs = [(1,0,0), (0,1,0), (0,0,1), (0,0,0), (1,1,1)]
      self.xs_observed = Counter()
      self.runTest(model, collection=shield_configs)

        
    
