from unittest import TestCase
from ctoybox import Toybox
from toybox.interventions.breakout import *
from toybox.interventions.core import get_property, Color, parse_property_access

class BreakoutGetProperty(TestCase):

  def test_get_property_simple(self):
    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        game = intervention.game

        bricks = get_property(game, 'bricks')
        self.assertIsInstance(bricks, BrickCollection)

        brick = get_property(game, 'bricks[1]')
        self.assertIsInstance(brick, Brick)

        second_brick_col = get_property(intervention.game, 'bricks[1].col')
        self.assertEqual(second_brick_col, 0)

  def test_set_property(self):
    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        game = intervention.game
        first_brick_r = get_property(game, 'bricks[0].color.r', setval=72)
        first_brick_g = get_property(game, 'bricks[0].color.g', setval=72)
        first_brick_b = get_property(game, 'bricks[0].color.b', setval=72)
        self.assertEqual(first_brick_r, 72)
        self.assertEqual(first_brick_g, 72)
        self.assertEqual(first_brick_b, 72)
        self.assertEqual(game.bricks[0].color.r, 72)
        self.assertEqual(game.bricks[0].color.g, 72)
        self.assertEqual(game.bricks[0].color.b, 72)
        self.assertNotEqual(game.bricks[1].color.r, 72)
        self.assertNotEqual(game.bricks[1].color.g, 72)
        self.assertNotEqual(game.bricks[1].color.b, 72)

  def test_set_property_zero(self):
    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        game = intervention.game
        v = get_property(game, 'bricks[107].points', setval=0)
        self.assertEqual(v, 0)
        

  def test_get_parent(self):
    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        game = intervention.game
        # you would only use this if you wanted to get the parent
        # AND set the child
        first_brick_color = get_property(game, 'bricks[0].color.r', setval=42, get_container=True)
        self.assertIsInstance(first_brick_color, Color)
        self.assertEqual(first_brick_color.r, 42)
        
        bkout = get_property(game, 'paddle_speed', setval=100., get_container=True)
        self.assertIsInstance(bkout, Breakout)
        self.assertEqual(bkout.paddle_speed, 100.)

  
  def test_set_at_index(self):
    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        game = intervention.game
        
        new_brick = Brick.decode(intervention, game.bricks[2].encode(), Brick)
        old_brick = game.bricks[1]
        self.assertNotEqual(new_brick, old_brick)
        foo = get_property(game, 'bricks[1]', setval=new_brick)
        self.assertIsInstance(foo, Brick)
        
        self.assertNotEqual(old_brick, foo) # not setting foo to be new?
        self.assertEqual(foo, new_brick)
  
  def test_property_parsing(self):
    example = 'abc.def[7][8].y[5]'
    self.assertListEqual(parse_property_access(example), ['abc', 'def', 7, 8, 'y', 5])