from toybox.interventions.base import *
from toybox.interventions.core import * 
try:
  import ujson as json
except:
  import json
import typing
"""An API for interventions on Breakout."""

class Breakout(Game):

  expected_keys = Game.expected_keys + ['paddle', 'is_dead', 'balls', 'ball_radius', 'paddle_speed', 'reset', 'bricks', 'paddle_width']

  immutable_fields = ['balls', 'bricks', 'intervention']

  def __init__(self, intervention, 
    score=None, lives=None, rand=None, level=None,
    paddle=None, paddle_width=None, paddle_speed=None,
    ball_radius=None, balls=None,
    bricks=None,
    reset=None, is_dead=None):

      super().__init__(intervention, score, lives, rand, level)
      self.paddle = Paddle.decode(intervention, paddle, Paddle)
      self.reset = reset
      self.ball_radius = ball_radius
      self.bricks = BrickCollection.decode(intervention, bricks, BrickCollection)
      self.balls = BallCollection.decode(intervention, balls, BallCollection)
      self.paddle_speed = paddle_speed
      self.paddle_width = paddle_width
      self.is_dead = is_dead
      self._in_init = False  

  def __eq__(self, other) -> Either:
    names = {
      'score'       : (self.score,        other.score), 
      'lives'       : (self.lives,        other.lives), 
      'level'       : (self.level,        other.level),
      'paddle'      : (self.paddle,       other.paddle),
      'reset'       : (self.reset,        other.reset),
      'ball_radius' : (self.ball_radius,  other.ball_radius) ,
      'bricks'      : (self.bricks,       other.bricks),
      'balls'       : (self.balls,        other.balls),
      'paddle_speed': (self.paddle_speed, other.paddle_speed),
      'paddle_width': (self.paddle_width, other.paddle_width),
      'is_dead'     : (self.is_dead,      other.is_dead)
    }
    return eq_map(names)

    def __str__(self):
        return """
Breakout
==========
    score: {}
    lives: {}
    level: {}
    paddle: {}
    reset: {}
    ball_radius: {}
    bricks: {}
    balls: {}
    paddle_speed: {}
    paddle_width: {}
    is_dead: {}""".format(self.score, self.lives, self.level, str(self.paddle), 
        self.reset, self.ball_radius, self.bricks.__str__(), self.balls.__str__(), self.paddle_speed,
        self.paddle_width, self.is_dead)

class Paddle(BaseMixin):

  expected_keys = ['velocity', 'position']
  immutable_fields = []  
  
  def __init__(self, intervention, velocity, position):
    super().__init__(intervention)
    self.velocity = Vec2D.decode(intervention, velocity, Vec2D)
    self.position = Vec2D.decode(intervention, position, Vec2D)
    self._in_init = False  
  
  def __eq__(self, other) -> Either:
    names = {
      'velocity': (self.velocity, other.velocity),
      'position': (self.position, other.position)
    }
    return eq_map(names)   
  
  def __str__(self):
      return '<position: {}, velocity: {}>'.format(self.position, self.velocity)


class BrickCollection(Collection):

  def __init__(self, intervention, bricks):
    super().__init__(intervention, bricks, Brick)
    self._in_init = False  

  def decode(intervention, bricks, clz):
    return BrickCollection(intervention, bricks)


class Brick(BaseMixin):

  expected_keys = ['destructible', 'depth', 'color', 'alive', 'points', 'size', 'position', 'row', 'col']
  immutable_fields = ['intervention']
    
  def __init__(self, intervention, destructible, depth, color, alive, points, size, position, row, col):
    super().__init__(intervention)
    self.destructible = destructible
    self.depth = depth
    self.color = Color.decode(intervention, color, Color)
    self.alive = alive
    self.points = points
    self.size = Vec2D.decode(intervention, size, Vec2D)
    self.position = Vec2D.decode(intervention, position, Vec2D)
    self.row = row
    self.col = col
    self._in_init = False

  def __eq__(self, other) -> Either:
    names = {
      'destructible': (self.destructible, other.destructible),
      'depth':        (self.depth,    other.depth),
      'color':        (self.color,    other.color),
      'alive':        (self.alive,    other.alive),
      'points':       (self.points,   other.points),
      'size':         (self.size,     other.size),
      'position':     (self.position, other.position),
      'row':          (self.row,      other.row),
      'col':          (self.col,      other.col)
    }
    return eq_map(names)


class BallCollection(Collection):

  def __init__(self, intervention, balls):
    super().__init__(intervention, balls, Ball)
    self._in_init = False  

  def __str__(self):
    if len(self) == 1:
      return str(self[0])
    else:
      return '[{}]'.format(', '.join(str(b) for b in self))


class Ball(BaseMixin): 

  expected_keys = ['position', 'velocity']
  immutable_fields = ['intervention']  

  def __init__(self, intervention, position, velocity):
    super().__init__(intervention)
    self.position = Vec2D.decode(intervention, position, Vec2D)
    self.velocity = Vec2D.decode(intervention, velocity, Vec2D)
    self._in_init = False  
  
  def __eq__(self, other) -> Either:
    names = {
        'position': (self.position, other.position),
        'velocity': (self.velocity, other.velocity)
    }
    return eq_map(names)
  
  def __str__(self):
    return 'Ball(position: {}, velocity: {})'.format(self.position, self.velocity)


class BreakoutIntervention(Intervention):

    def __init__(self, tb, game_name='breakout'):
        # check that the simulation in tb matches the game name.
        Intervention.__init__(self, tb, game_name, Breakout)

    def num_bricks_remaining(self):
        return sum([int(brick.alive) for brick in self.game.bricks])

    def num_bricks(self):
        return len(self.game.bricks)

    def num_rows(self):
        return len(self.config['row_scores'])

    def num_columns(self):
        """Returns the number of columns in the layout."""
        rows = self.num_rows()
        bricks = self.num_bricks()
        return bricks // rows

    def add_row(self, bricks, points, pre=None, post=None):
        """Adds the input row of bricks to the playing board.

        Parameters
        ====
        bricks: a list of brick objects
        value: the points associated with this row
        pre: add the list above
        post: add the list below
        """

        input_len = len(bricks)
        target_len = self.num_bricks()

        if input_len != target_len:
            raise ValueError('Input brick list length incorrect (is %d; should be %d)' % (input_len, target_len))

        if pre:
            for brick in bricks.reverse():
                self.bricks.insert(0, brick)
        
        elif post: 
            self.bricks.extend(bricks)

        else:
            raise ValueError('Must provide one optional argument: pre or post.')

        self.config['row_scores'].append(points)
        self.dirty_config = True

    def is_stack(self, bricks):
        col = bricks[0].col
        return all([b.col == col for b in bricks])

    def is_channel(self, bricklist):
        """Predicate indicating whether the input list of bricks constitutes a channel."""
        col = bricklist[0].col
        for brick in bricklist:
            if brick.col != col: return False
            if brick.alive: return False
        return True

    def get_column(self, i):
        """Returns the ith column of bricks."""
        bricks = []
        for brick in self.game.bricks:
            if brick.col == i:
                bricks.append(brick)
        return bricks
    
    def channel_count(self):
        count = 0
        for i in range(self.num_columns()):
            channel = self.get_column(i)
            if self.is_channel(channel): count += 1
        return count

    def get_ball_position(self):
        """Returns a list of positions, if there is more than one ball, and a single Vec2D object otherwise.:"""
        nballs = len(self.game.balls)
        if nballs > 1:
            return [ball.position for ball in self.game.balls]
        else:  
            return self.game.balls[0].position

    def get_ball_velocity(self):
        nballs = len(self.game.balls)
        if nballs > 1:
            return [ball.velocity for ball in self.game.balls]
        else:  
            return self.game.balls[0].velocity

    def get_paddle_position(self):
        return self.game.paddle.position

    def get_paddle_velocity(self):
        return self.game.paddle.velocity

    def find_brick(self, pred):
        for i, b in enumerate(self.game.bricks):
            if pred(b):
                return i, b
        raise ValueError('No bricks that satisfy the input predicate found.')

    def add_channel(self, i):
        """Turns the ith column into a channel"""
        for brick in self.game.bricks:
            if brick.col == i and brick.alive:
                brick.alive = False

    def fill_column(self, i): 
        """Fills the ith column, so that all bricks are now alive."""
        for brick in self.game.bricks:
            if brick.col == i and not brick.alive:
                brick.alive = True

    def find_channel(self):
        """Returns the first channel found."""
        for i in range(self.num_columns()):
            col = self.get_column(i)
            if self.is_channel(col):
                return i, col
        return -1, None

    def clear_board(self):
        """Clears the board of all bricks"""
        for brick in self.game.bricks:
            brick.alive = False


if __name__ == "__main__":
  import argparse 
  from ctoybox import Toybox, Input

  parser = argparse.ArgumentParser(description='test Amidar interventions')
  parser.add_argument('--partial_config', type=str, default="null")
  parser.add_argument('--save_json', type=bool, default=False)
  args = parser.parse_args()

  with Toybox('breakout') as tb:

    fire = Input()
    fire.button1 = True
    noop = Input()
    tb.apply_action(fire)

    state = tb.to_state_json()
    config = tb.config_to_json()

    if args.save_json:
        # save a sample starting state and config
        with open('toybox/toybox/interventions/defaults/breakout_state_default.json', 'w') as outfile:
            json.dump(state, outfile)

        with open('toybox/toybox/interventions/defaults/breakout_config_default.json', 'w') as outfile:
            json.dump(config, outfile)

    with BreakoutIntervention(tb) as intervention:
        intervention.game.lives = 1
        assert intervention.dirty_state
    
    # remove and assert that the brick is gone
    with BreakoutIntervention(tb) as intervention:
        nbricks = intervention.num_bricks_remaining()
        intervention.game.bricks[0].alive = False
        nbricks_post = intervention.num_bricks_remaining()

        assert nbricks - 1 == nbricks_post

    # reset and assert that the brick is present
    with BreakoutIntervention(tb) as intervention:
        nbricks = intervention.num_bricks_remaining()
        intervention.game.bricks[0].alive = True
        nbricks_post = intervention.num_bricks_remaining()

        assert nbricks + 1 == nbricks_post

    # add a channel and assert that num_rows bricks have been removed
    with BreakoutIntervention(tb) as intervention: 
        nbricks = intervention.num_bricks_remaining()
        intervention.add_channel(0)
        nbricks_post = intervention.num_bricks_remaining()
        assert nbricks_post == nbricks - intervention.num_rows()

        col, channel = intervention.find_channel()
        assert channel

        assert intervention.channel_count() == 1

    # remove a channel and assert that num_rows bricks have been added
    with BreakoutIntervention(tb) as intervention: 
        nbricks = intervention.num_bricks_remaining()
        intervention.fill_column(0)
        nbricks_post = intervention.num_bricks_remaining()
        assert nbricks_post == nbricks + intervention.num_rows()

    # get ball position, even when multiple balls present
    with BreakoutIntervention(tb) as intervention: 
        game = intervention.game
        assert len(game.balls) > 0

        ball = game.balls[0]
        game.balls.append(ball)
        ball_positions = intervention.get_ball_position()
        assert len(ball_positions) == 2
        ball_velocities = intervention.get_ball_velocity()
        assert len(ball_velocities) == 2
        game.balls.clear()
        game.balls.append(ball)
        # the line above should have triggered an error
        ball_positions = intervention.get_ball_position()

    # move ball diagonally by sqrt(2) pixels
    with BreakoutIntervention(tb) as intervention: 
        ball_pos = intervention.get_ball_position()
        ball_pos.x = ball_pos.x + 1
        ball_pos.y = ball_pos.y + 1
    with BreakoutIntervention(tb) as intervention: 
        ball_pos_post = intervention.get_ball_position()
        assert ball_pos_post.x == ball_pos.x
        ball_pos_post.x = ball_pos.x - 1
        ball_pos_post.y = ball_pos.y - 1
    with BreakoutIntervention(tb) as intervention: 
        ball_pos_post_post = intervention.get_ball_position()
        assert ball_pos_post.x == ball_pos_post_post.x


    # change ball velocity
    with BreakoutIntervention(tb) as intervention: 
        ball_vel = intervention.get_ball_velocity()
        ball_vel.x = ball_vel.x + 1
        ball_vel.y = ball_vel.y + 1
        ball_vel_post = intervention.get_ball_velocity()
        assert ball_vel_post.x == ball_vel.x

        ball_vel.x = ball_vel.x - 1
        ball_vel.y = ball_vel.y - 1
        ball_vel_post = intervention.get_ball_velocity()
        assert ball_vel_post.x == ball_vel.x

    # get paddle position and move
    with BreakoutIntervention(tb) as intervention: 
        pos = intervention.get_paddle_position()
        assert pos.x == 120.0 and pos.y == 143.0

        pos.x = pos.x + 10
        pos_post = intervention.get_paddle_position()
        assert pos.x == pos_post.x
