import toybox.testing.models.openai_baselines as oai
import toybox.interventions.breakout as brk

import tensorflow as tf

import os

from abc import abstractmethod
from toybox.sample_tests.base import BreakoutToyboxTestBase
    
class EZChannel(BreakoutToyboxTestBase):

    def setUp(self):
        super().setUp(trials=3, timeout=500)

    def shouldIntervene(self, obj=None):
        return self.tick == 0

    def intervene(self, obj):
        with brk.BreakoutIntervention(self.getToybox()) as intervention:
            game = intervention.game
            intervention.game.lives = 1
            # Make sure we only have one ball; it appears that the game currently 
            # has two balls
            # # Grab the column to clear
            col = obj.col
            row = obj.row
            intervention.add_channel(col)
            # # Now grab the brick associated with the current target, and make it live again
            _, brick = intervention.find_brick(lambda b: b.col == col and b.row == row)
            brick.alive = True
            assert intervention.dirty_state
        
    
    def test_ezchannel_ppo2(self):
        seed = 8675309
        path = '../models/BreakoutToyboxNoFrameskip-v4.regress.model'
        with brk.BreakoutIntervention(self.getToybox()) as intervention:
            bricks = intervention.game.bricks
        with tf.Session(graph=tf.Graph()):
            model = oai.getModel(self.env, 'ppo2', seed, path)
            self.runTest(model, collection=bricks)

    def onTestEnd(self):
        print('test end!')

    def onTrialEnd(self):
        print('trial end!')
        
