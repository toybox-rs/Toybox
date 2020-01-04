from toybox.sample_tests.base import SpaceInvadersToyboxTestBase
import toybox.interventions.space_invaders as si
import toybox.testing.models.openai_baselines as oai


seed = 1984
path = '../models/SpaceInvadersToyboxNoFrameskip-v4.regress.model'

class NoShields(SpaceInvadersToyboxTestBase):

    def setUp(self): super().setUp(trials=5, timeout=500)

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