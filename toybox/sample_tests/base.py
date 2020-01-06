import toybox.testing.behavior as behavior
import toybox.testing.envs.gym as gym
import toybox.testing.models.openai_baselines as oai


class ToyboxTestBase(behavior.BehavioralFixture):
  
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
      if self.hasTimedOut():
        self.final_state = self.toybox.to_state_json()
        return True 
      # Only set if we haven't already -- e.g., if we've died, but 
      # the game isn't over yet. DO NOT override the state that's passed 
      # we actually see a game over.
      elif self.done and not self.final_state:
        self.final_state = self.toybox.to_state_json()
      return self.done

    def setUp(self, **kwargs): super().setUp(trials=2, timeout=500)


class AmidarToyboxTestBase(ToyboxTestBase):

    @classmethod
    def setUpEnv(cls):
      seed = 0xdeadbeef
      gym.setUpToyboxGym(cls, 'AmidarToyboxNoFrameskip-v4', seed)

class BreakoutToyboxTestBase(ToyboxTestBase):
    
    @classmethod
    def setUpEnv(cls):
        seed = 8675309
        gym.setUpToyboxGym(cls, 'BreakoutToyboxNoFrameskip-v4', seed)

class SpaceInvadersToyboxTestBase(ToyboxTestBase):

    @classmethod
    def setUpEnv(cls):
      seed = 42
      gym.setUpToyboxGym(cls, 'SpaceInvadersToyboxNoFrameskip-v4', seed)