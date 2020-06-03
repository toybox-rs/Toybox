from unittest import TestCase
from ctoybox import Toybox, Input
from toybox.interventions.breakout import * 
from toybox.interventions.base import ProbEq, MutationError

import json, os
from copy import copy

class BreakoutSample(TestCase):

  def setUp(self):
    self.tb = Toybox('breakout')
    fire = Input()
    fire.button1 = True
    noop = Input()
    self.tb.apply_action(fire)

    if os.path.exists('breakout_models'):
      self.data = []
    else:
      print('Running setUp (this can take some time)')

      # load up some data and test sampling
      # test for just one run
      data = []
      datadir = 'toybox/interventions/models/data/StayAlive'
      for seed in os.listdir(datadir):
        this_dir = datadir + os.sep + seed
        for dat in os.listdir(this_dir)[:100]:
          this_file = this_dir + os.sep + dat
          if this_file.endswith('json'):
            with open(this_file, 'r') as f:
              data.append(Breakout.decode(BreakoutIntervention(self.tb), json.load(f), Breakout))
      self.data = data
      print('Finished setUp!')

  def test_sample_simple(self):
    with BreakoutIntervention(self.tb, modelmod='breakout_models', data=self.data) as intervention:
      intervention.eq_mode = ProbEq
      game = intervention.game 

      sample1 = game.sample('ball_radius')
      sample2 = game.sample('ball_radius')
      self.assertIsInstance(sample1, Breakout)
      self.assertNotEqual(sample1.ball_radius, sample2.ball_radius)

      eq_check = sample1 == sample2
      varname, new, old = eq_check.differ
      self.assertIsInstance(eq_check, ProbEq)
      self.assertEqual(varname, 'ball_radius')
      self.assertNotAlmostEqual(new, old)
        
      sample3 = game.sample('paddle_speed', 'paddle_width')
      eq_check = sample3 == game
      varname, new, old = eq_check.differ
      self.assertIsInstance(eq_check, ProbEq)
      self.assertTrue(varname == 'paddle_speed' or varname == 'paddle_width')
      self.assertNotAlmostEqual(new, old)

      sample4 = game.sample('paddle')
      eq_check = sample4 == game
      varname, new, old = eq_check.differ      
      self.assertIsInstance(eq_check, ProbEq)
      # The equality check will return the first leaf that differs
      self.assertTrue('paddle' in varname)
      self.assertNotAlmostEqual(new, old)

      sample5 = game.sample('bricks')
      self.assertNotEqual(sample5, game)

      sample6 = Breakout.decode(intervention, game.encode(), Breakout)
      self.assertEqual(sample6, game)

      sample6 = sample6.sample('bricks[0]')
      self.assertNotEqual(sample6, game)

      varold = None
      for i in range(len(sample6.bricks))[1:]:
        sample7 = sample6.sample('bricks[%d]' % i)
        self.assertNotEqual(sample6, sample7)
        eq_check = sample6 == sample7
        vardiff = eq_check.differ[0]
        if varold:
          self.assertNotEqual(varold, vardiff)
        varold = vardiff

        sample6 = sample7
        self.assertNotEqual(sample6, game)
      

      eq_check1 = sample6 == game
      eq_check2 = sample6 == game
      varname1, new1, old1 = eq_check1.differ
      varname2, new2, old2 = eq_check2.differ
      self.assertNotEqual(len(sample6.bricks), 1)
      self.assertIsInstance(eq_check1, ProbEq)
      self.assertNotEqual(varname1, varname2)
      self.assertNotAlmostEqual(new1, old1)
      self.assertNotAlmostEqual(new2, old2)

  def test_sample_ball(self):
    with BreakoutIntervention(self.tb, modelmod='breakout_models', data=self.data) as intervention:
      ball = intervention.game.balls[0]
      vx_old = ball.velocity.x
      vx_new = intervention.game.sample('balls[0].velocity.x').balls[0].velocity.x
      self.assertNotAlmostEqual(vx_old, vx_new)