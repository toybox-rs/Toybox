from unittest import TestCase
from ctoybox import Toybox, Input
import toybox.interventions.breakout as breakout
from toybox.interventions.breakout import BreakoutIntervention, Breakout
from toybox.interventions.base import MutationError, InterventionNoneError

class BreakoutInterventionTests(TestCase):

  def setUp(self):
    self.tb = Toybox('breakout')

    fire = Input()
    fire.button1 = True
    noop = Input()
    self.tb.apply_action(fire)

  def test_allowable_interventions(self):
    with BreakoutIntervention(self.tb) as intervention:
      with self.assertRaises(InterventionNoneError):
        intervention.game.paddle.intervention = None

      with self.assertRaises(MutationError):
        intervention.game.paddle.intervention = intervention

      with self.assertRaises(MutationError):
        intervention.game.paddle._in_init = True
          # assert False, 'We should not be able to manulaly set the _in_init flag'

      self.assertIn('intervention', intervention.game.paddle.immutable_fields)
      self.assertNotIn('_in_init', intervention.game.paddle.immutable_fields)


  def test_dirty_state(self):
    with BreakoutIntervention(self.tb) as intervention:
      intervention.game.lives = 1
      self.assertTrue(intervention.dirty_state)
      self.assertFalse(intervention.dirty_config)

  def test_not_dirty_state(self):
    with BreakoutIntervention(self.tb) as intervention:
      lives = intervention.game.lives
      self.assertFalse(intervention.dirty_state)
      self.assertFalse(intervention.dirty_config)
    
  def test_removal_from_collection(self):
    with BreakoutIntervention(self.tb) as intervention:
      nbricks = intervention.num_bricks_remaining()
      intervention.game.bricks[0].alive = False
      nbricks_post = intervention.num_bricks_remaining()

      self.assertEqual(nbricks_post, nbricks - 1)

    # reset and assert that the brick is present
    with BreakoutIntervention(self.tb) as intervention:
      nbricks = intervention.num_bricks_remaining()
      intervention.game.bricks[0].alive = True
      nbricks_post = intervention.num_bricks_remaining()
      self.assertEqual(nbricks_post, nbricks + 1)

  def test_channel_manipulation(self):
    # add a channel and assert that num_rows bricks have been removed
    with BreakoutIntervention(self.tb) as intervention: 
      nbricks = intervention.num_bricks_remaining()
      intervention.add_channel(0)
      nbricks_post = intervention.num_bricks_remaining()
      self.assertEqual(nbricks_post, nbricks - intervention.num_rows())
      col, channel = intervention.find_channel()
      self.assertIsNotNone(channel)
      self.assertEqual(1, intervention.channel_count())

    # remove a channel and assert that num_rows bricks have been added
    with BreakoutIntervention(self.tb) as intervention: 
      nbricks = intervention.num_bricks_remaining()
      intervention.fill_column(0)
      nbricks_post = intervention.num_bricks_remaining()
      self.assertEqual(nbricks_post, nbricks + intervention.num_rows())

  def test_brick_color_change(self):
    with BreakoutIntervention(self.tb) as intervention:
      b50g_from = intervention.game.bricks[50].color.g
      intervention.game.bricks[50].color.g = 77
      b50g_to = intervention.game.bricks[50].color.g

    with BreakoutIntervention(self.tb) as intervention:
      # make sure it was written to the game
      b50g_to_check = intervention.game.bricks[50].color.g

    self.assertNotEqual(b50g_from, b50g_to)
    self.assertEqual(b50g_to, b50g_to_check)

  def test_get_ball_position(self):
    # get ball position, even when multiple balls present
    with BreakoutIntervention(self.tb) as intervention: 
      game = intervention.game
      self.assertGreater(len(game.balls), 0)
      ball = game.balls[0]
      game.balls.append(ball)
      ball_positions = intervention.get_ball_position()
      self.assertEqual(len(ball_positions), 2)
      ball_velocities = intervention.get_ball_velocity()
      self.assertEqual(len(ball_velocities), 2)
      game.balls.clear()
      game.balls.append(ball)
      # the line above should have triggered an error
      ball_positions = intervention.get_ball_position()

  def test_move_diagonally(self):
    # move ball diagonally by sqrt(2) pixels
    with BreakoutIntervention(self.tb) as intervention: 
      ball_pos = intervention.get_ball_position()
      ball_pos.x = ball_pos.x + 1
      ball_pos.y = ball_pos.y + 1

    with BreakoutIntervention(self.tb) as intervention: 
      ball_pos_post = intervention.get_ball_position()
      self.assertEqual(ball_pos_post.x, ball_pos.x)
      ball_pos_post.x = ball_pos.x - 1
      ball_pos_post.y = ball_pos.y - 1

    with BreakoutIntervention(self.tb) as intervention: 
      ball_pos_post_post = intervention.get_ball_position()
      self.assertEqual(ball_pos_post.x, ball_pos_post_post.x)

  def test_change_ball_velocity(self):
    # change ball velocity
    with BreakoutIntervention(self.tb) as intervention: 
      ball_vel = intervention.get_ball_velocity()
      ball_vel.x = ball_vel.x + 1
      ball_vel.y = ball_vel.y + 1
      ball_vel_post = intervention.get_ball_velocity()
      self.assertEqual(ball_vel_post.x, ball_vel.x)
      ball_vel.x = ball_vel.x - 1
      ball_vel.y = ball_vel.y - 1
      ball_vel_post = intervention.get_ball_velocity()
      self.assertEqual(ball_vel_post.x, ball_vel.x)

  def test_move(self):
    # get paddle position and move
    with BreakoutIntervention(self.tb) as intervention: 
      pos = intervention.get_paddle_position()
      self.assertAlmostEqual(pos.x, 120.0)
      self.assertAlmostEqual(pos.y, 143.0)
      pos.x = pos.x + 10
      pos_post = intervention.get_paddle_position()
      self.assertAlmostEqual(pos.x, pos_post.x)

