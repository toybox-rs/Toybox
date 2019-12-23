import toybox.testing.behavior as behavior
import toybox.testing.envs.gym as gym
import toybox.interventions.amidar as ami
import toybox.testing.models.openai_baselines as oai
from abc import ABC, abstractmethod

class AmidarToyboxTest(behavior.BehavioralFixture):

    @classmethod
    def setUpEnv(cls):
      seed = 0xdeadbeef
      gym.setUpToyboxGym(cls, 'AmidarToyboxNoFrameskip-v4', seed)
    
    @classmethod
    def tearDownEnv(cls):
      gym.tearDownToyboxGym(cls)

    def takeAction(self, model):
      oai.takeAction(self, model)

    def stepEnv(self):
      gym.stepEnv(self)

    def resetEnv(self):
      gym.resetEnv(self)

    def isDone(self):
      lives = self.toybox.get_lives()
      has_reset = lives > self.lives
      self.lives = lives
      return self.hasTimedOut() or has_reset

behavior.BehavioralFixture.register(AmidarToyboxTest)
