import toybox.testing.models.openai_baselines as oai
import toybox.interventions.breakout as brk

import tensorflow as tf

import os

from abc import abstractmethod
from toybox.sample_tests.base import BreakoutToyboxTestBase
    
class EZChannel(BreakoutToyboxTestBase):

    def setUp(self):
        super().setUp(trials=1, timeout=500)

    def shouldIntervene(self, obj=None):
        return self.tick == 0

    # def subTest(self, obj=None):
    #     self.resetEnv()
    #     return super().subTest(obj)

    def intervene(self, obj):
        with brk.BreakoutIntervention(self.getToybox()) as intervention:
            import json 
            game = intervention.game
            with open('before.json', 'w') as f1:
                f1.write(json.dumps(game.encode()))
            print('about to set lives...')
            intervention.game.lives = 1
            assert intervention.dirty_state 
            # assert intervention.dirty_state
        #     # Make sure we only have one ball; it appears that the game currently 
        #     # has two balls
        #     # # Grab the column to clear
        #     #col = obj.col
        #     #row = obj.row
        #     #intervention.add_channel(1)
        #     game.balls.clear()
        #     game.bricks[0].alive = False
        #     # # Now grab the brick associated with the current target, and make it live again
        #     # _, brick = intervention.find_brick(lambda b: b.col == col and b.row == row)
        #     # brick.alive = True
        #     #assert intervention.dirty_state
        with open('after.json', 'w') as f2:
            f2.write(json.dumps(game.encode()))
        
    
    def test_ezchannel_ppo2(self):
        seed = 8675309
        path = '../models/BreakoutToyboxNoFrameskip-v4.regress.model'
        with tf.Session(graph=tf.Graph()):
            model = oai.getModel(self.env, 'ppo2', seed, path)
            self.runTest(model)

    def onTestEnd(self):
        print('test end!')

    def onTrialEnd(self):
        print('trial end!')
        
