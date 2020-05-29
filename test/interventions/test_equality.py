from unittest import TestCase
from ctoybox import Toybox, Input
import toybox.interventions.breakout as breakout
from toybox.interventions.breakout import BreakoutIntervention, Breakout
from toybox.interventions.base import ProbEq, SetEq

class BreakoutEquality(TestCase):

  def test_standard_eq(self):
    s1, s2, s3 = None, None, None
    with Toybox('breakout') as tb:

      fire = Input()
      fire.button1 = True
      noop = Input()
      tb.apply_action(fire)

      with BreakoutIntervention(tb) as intervention:
        s1 = intervention.game

      with BreakoutIntervention(tb) as intervention:
        s2 = intervention.game

      with BreakoutIntervention(tb) as intervention:
        intervention.game.paddle_speed = 10
        s3 = intervention.game
    
    self.assertEqual(s1, s2)
    self.assertNotEqual(s1, s3)
    self.assertNotEqual(s2, s3)

  def test_prob_eq_atomic_manipulation(self):
    s1, s2, s3 = None, None, None
    with Toybox('breakout') as tb:

      fire = Input()
      fire.button1 = True
      noop = Input()
      tb.apply_action(fire)

      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = ProbEq
        s1 = intervention.game

      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = ProbEq
        s2 = intervention.game

      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = ProbEq
        intervention.game.paddle_speed = 10.
        s3 = intervention.game
    
    self.assertEqual(s1, s2)
    self.assertNotEqual(s1, s3)
    self.assertNotEqual(s2, s3)

    result = s1 == s3
    self.assertIsInstance(result, ProbEq)
    self.assertEqual(result.differ[0], 'paddle_speed')

  def test_prob_eq_compound_manipulation(self):
    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = ProbEq
        initial_state = intervention.game

    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = ProbEq
        # alter the x value of all bricks 
        for brick in intervention.game.bricks:
          brick.position.x += 0.1
        intervened = intervention.game
      
    self.assertNotEqual(initial_state, intervened)
    cmp1 = initial_state == intervened
    cmp2 = initial_state == intervened
    self.assertNotEqual(cmp1, cmp2)
    self.assertNotEqual(cmp1.differ, cmp2.differ)

    with Toybox('breakout') as tb:
      fire = Input()
      fire.button1 = True
      noop = Input()
      tb.apply_action(fire)

      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = ProbEq
        for brick in intervention.game.bricks[:25]:
          brick.position.x += 1
        intervention.game.paddle_speed = 10.
        intervention.game.paddle.velocity.x = 12.
        intervention.game.balls.append(intervention.game.balls[0])
        intervened = intervention.game

    self.assertNotEqual(initial_state, intervened)
    cmp1 = initial_state == intervened
    cmp2 = initial_state == intervened
    self.assertNotEqual(cmp1, cmp2)
    self.assertNotEqual(cmp1.differ, cmp2.differ)
    if len(cmp1.key_order) and len(cmp2.key_order):
      self.assertNotEqual(cmp1.key_order[0], cmp2.key_order[0])

  def test_set_eq(self): 
    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = SetEq
        initial_state = intervention.game
    
    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = SetEq
        initial_state_copy = intervention.game
    
    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = SetEq
        intervened1 = intervention.game
        intervention.game.paddle_speed += 1
        intervention.game.lives += 1

    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = SetEq
        intervened2 = intervention.game
        intervention.game.paddle_speed += 1
        intervention.game.lives +=1

    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb) as intervention:
        intervention.eq_mode = SetEq
        intervened3 = intervention.game 
        intervention.game.paddle_speed += 2
        intervention.game.lives += 1

    # print(initial_state == initial_state_copy)
    self.assertEqual(initial_state, initial_state_copy)
    self.assertNotEqual(initial_state, intervened1)
    # print(initial_state == intervened1)
    self.assertEqual(intervened1, intervened2)
    # print(intervened1==intervened2)
    self.assertNotEqual(intervened1, intervened3)
    # print(intervened1==intervened3)

  def test_set_eq_prop_state(self):
    s1, s2 = None, None

    with Toybox('breakout') as tb:
      with BreakoutIntervention(tb, eq_mode=SetEq) as intervention:
        s1 = Breakout.decode(intervention, intervention.game.encode(), Breakout)
        intervention.game.bricks[50].color.g = 99
        s2 = Breakout.decode(intervention, intervention.game.encode(), Breakout)

    self.assertEqual(s1.bricks[49].color.g, s2.bricks[49].color.g)
    self.assertNotEqual(s1.bricks[50].color.g, s2.bricks[50].color.g)

    self.assertEqual(s1.bricks[49].color, s2.bricks[49].color)
    self.assertNotEqual(s1.bricks[50].color, s2.bricks[50].color)
    
    self.assertEqual(s1.bricks[49], s2.bricks[49])
    self.assertNotEqual(s1.bricks[50], s2.bricks[50])

    self.assertNotEqual(s1.bricks, s2.bricks)
    self.assertNotEqual(s1, s2)

