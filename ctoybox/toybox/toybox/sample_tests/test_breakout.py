import toybox.testing.envs.gym as gym
import toybox.testing.models.openai_baselines as oai
import toybox.interventions.breakout as brk
import toybox.testing.behavior as behavior

import os

from abc import abstractmethod

class BreakoutToyboxTestBase(behavior.BehavioralFixture):

    @classmethod
    def setUpEnv(cls):
        seed = 8675309
        gym.setUpToyboxGym(cls, 'AmidarToyboxNoFrameskip-v4', seed)

    @classmethod
    def tearDownEnv(cls):
        gym.tearDownToyboxGym(cls)

    def takeAction(self, model):
        oai.takeAction(self, model)

    def stepEnv(self, action):
        gym.stepEnv(self.env, action)

    def resetEnv(self):
        gym.resetEnv(self)
    
    def isDone(self):
        lives = self.toybox.get_lives()
        level = self.toybox.get_level()
        has_reset = lives > self.lives
        self.lives = lives
        return self.hasTimedOut() or has_reset or lives < 1 or level != 1

    @abstractmethod
    def intervene(self): pass
    
class EZChannel(BreakoutToyboxTestBase):

    def shouldIntervene(self):
        return self.tick == 0


    def intervene(self):
        with brk.BreakoutIntervention(self.getToybox()) as intervention:
            # Limit to one life
            intervention.game.lives = 1
            # Grab the column to clear
            col = self.target.col
            intervention.add_channel(col)
            # Now grab the brick associated with the current target, and make it live again
            brick = intervention.game.find_brick(lambda b: b.col == col and b.row == self.target.row)
            brick.alive = True

    def subTest(self, obj=None):
        assert isinstance(obj, Brick), 'Expecting to iterate over Brick objects, not %s objects' % type(obj)
        self.target = obj
    
    def test_ezchanel_ppo2(self):
        seed = 8675309
        path = '../models/BreakoutToyboxNoFrameskip-v4.regress.model'
        model = oai.getModel(self.env, 'ppo2', seed, path)
        self.runTest(model, collection=self.game.bricks)

    def onTestEnd(self):
        print('test end!')

    def onTrialEnd(self):
        print('trial end!')
        
