import toybox.testing.models.openai_baselines as oai
import toybox.interventions.breakout as brk

import tensorflow as tf

import math
import os

from abc import abstractmethod
from toybox.sample_tests.base import BreakoutToyboxTestBase
from numpy import random

seed = 8675309
path = 'models/BreakoutToyboxNoFrameskip-v4.regress.model'        

class PolarStarts(BreakoutToyboxTestBase):

    def shouldIntervene(self, obj=None): return self.tick==0

    def onTestEnd(self): pass

    def onTrialEnd(self): 
        print('Ticks until death: %d' % self.tick)
        return self.tick

    def intervene(self, obj=None):
        angle = obj
        with brk.BreakoutIntervention(self.getToybox()) as intervention:
            ball = intervention.game.balls[0]
            intervention.game.balls.clear()
            intervention.game.balls.append(ball)

            ball_speed_slow = intervention.config['ball_speed_slow']
            velocity_y = ball_speed_slow * math.sin(math.radians(angle))
            velocity_x = ball_speed_slow * math.cos(math.radians(angle))
            if abs(velocity_y) < 0.0001:
                print('Skip angle=', angle, 'velocity_y=', velocity_y)
                return
            ball.velocity.y = velocity_y
            ball.velocity.x = velocity_x

    def test_polar_starts_regress(self):
        with tf.Session(graph=tf.Graph()):
            model = oai.getModel(self.env, 'ppo2', seed, path)
            degs = random.choice(range(0,360,5), 2)
            self.runTest(model, collection=degs)

    
class LastBrick(BreakoutToyboxTestBase):

    def shouldIntervene(self, obj=None): return self.tick==0

    def onTestEnd(self): pass

    def onTrialEnd(self): 
        print('Ticks until death: %d' % self.tick)

    def intervene(self, obj):
        with brk.BreakoutIntervention(self.getToybox()) as intervention:
            solo_ball = intervention.game.balls.pop()
            intervention.game.balls.clear()
            intervention.game.balls.append(solo_ball)
            intervention.clear_board()
            row = obj.row
            col = obj.col
            _, brick = intervention.find_brick(lambda b: b.row == row and b.col == col)
            brick.alive = True
            
    def test_last_brick_ppo2(self):
        seed = 8675309
        path = 'models/BreakoutToyboxNoFrameskip-v4.regress.model'
        with brk.BreakoutIntervention(self.getToybox()) as intervention:
            bricks = intervention.game.bricks
            intervention.game.reset = False
        with tf.Session(graph=tf.Graph()):
            model = oai.getModel(self.env, 'ppo2', seed, path)
            bricks = random.choice(bricks, 2)
            self.runTest(model, collection=bricks)


class EZChannel(BreakoutToyboxTestBase):

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
        path = 'models/BreakoutToyboxNoFrameskip-v4.regress.model'
        with brk.BreakoutIntervention(self.getToybox()) as intervention:
            bricks = intervention.game.bricks
        with tf.Session(graph=tf.Graph()):
            model = oai.getModel(self.env, 'ppo2', seed, path)
            bricks = random.choice(bricks, 2)
            self.runTest(model, collection=bricks)

    def onTestEnd(self):
        print('test end!')

    def onTrialEnd(self):
        print('trial end!')
        
