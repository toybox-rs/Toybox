from toybox.interventions.base import *
import json
"""An API for interventions on Breakout."""

class BreakoutIntervention(Intervention):

    def __init__(self, tb, game_name='breakout'):
        # check that the simulation in tb matches the game name.
        Intervention.__init__(self, tb, game_name)

    def num_bricks_remaining(self):
        return sum(self.state['bricks'][i]['alive'] for i in range(self.num_bricks()))
    

    def channel_count(self):
        stacks = self.get_stacks()
        return sum(self.is_channel(stack) for stack in stacks)
    

    def get_bricks(self):
        return self.state['bricks']


    def brick_alive(self, i):
        return self.state['bricks'][i]['alive']


    def num_bricks(self):
        return len(self.state['bricks'])


    def get_ball_position(self):
        nballs = len(self.state['balls'])
        if nballs > 1:
            return [self.state['balls'][i]["position"] for i in range(nballs)]
        else:  
            return self.state['balls'][0]["position"]


    def get_ball_velocity(self):
        nballs = len(self.state['balls'])
        if nballs > 1:
            return [self.state['balls'][i]["velocity"] for i in range(nballs)]
        else:  
            return self.state['balls'][0]["velocity"]


    def is_stack(self, intlist):
        ncols = self.num_columns()
        nrows = self.num_rows()
        offset = intlist[0] % ncols
        row = 1
        for i in intlist[1:]:
            if i != row * ncols + offset:
                return False
            row += 1
        return True 


    def find_brick(self, pred):
        for i, b in enumerate(self.state['bricks']):
            if pred(b):
                return i, b
        raise ValueError('No bricks that satisfy the input predicate found.')


    def num_rows(self):
        return len(self.config['row_scores'])


    def num_columns(self):
        rows = self.num_rows()
        bricks = self.num_bricks()
        return bricks // rows


    def get_stacks(self):
        """Returns a list of lists. Each element returned is a list of indices, 
        corresponding to a potential channel."""
        ncols = self.num_columns()
        nrows = self.num_rows()
        stacks = []
        for offset in range(ncols):
            stack = []
            for row in range(nrows):
                stack.append(row * ncols + offset)
            stacks.append(stack)
        return stacks


    def is_channel(self, intlist):
        bricks = self.get_bricks()
        return self.is_stack(intlist) and all([not self.brick_alive(i) for i in intlist])


    def get_paddle_position(self): 
        return self.state['paddle']['position']


    def set_brick(self, index, alive=True):
        self.state['bricks'][index]['alive'] = alive


    def find_channel(self):
        stacks = self.get_stacks()
        for i, stack in enumerate(stacks):
            if self.is_channel(stack):
                return i, stack
        raise ValueError('No channels found.')


    def add_channel(self, intlist):
        assert(self.is_stack(intlist))
        for i in intlist:
            self.set_brick(i, alive=False)


    def fill_channel(self, intlist): 
        assert(self.is_stack(intlist))
        for i in intlist: 
            self.set_brick(i, alive=True)


    # pos should contain a dictionary containing keys 'x' and 'y' for each ball 
    def set_ball_position(self, pos):
        if isinstance(pos, dict): 
            pos = [pos]

        nballs = len(self.state['balls'])
        assert len(self.state['balls']) == len(pos)

        for i in range(nballs): 
            assert isinstance(pos[i], dict)

            for k in ['x', 'y']:
                assert k in pos[i].keys()
                self.state['balls'][i]['position'][k] = pos[i][k]

    # vel should contain a dictionary containing keys 'x' and 'y' for each ball 
    def set_ball_velocity(self, vel):
        if isinstance(vel, dict): 
            vel = [vel]

        nballs = len(self.state['balls'])
        assert len(self.state['balls']) == len(vel)

        for i in range(nballs): 
            assert isinstance(vel[i], dict)

            for k in ['x', 'y']:
                assert k in vel[i].keys()
                self.state['balls'][i]['velocity'][k] = vel[i][k]


    def set_paddle_position(self, pos): 
        assert isinstance(pos, dict)
        for k in ['y', 'x']: assert k in pos.keys()

        self.state['paddle']['position']['y'] = pos['y'] 
        self.state['paddle']['position']['x'] = pos['x']

    
# Intervention tests
    # all the constants!
    # vanishing ball: after hitting the ball, make the ball disappear 
        # requires the model you throw at it has memory/sense of time
    # add brick in arbitrary location?


    def get_json(self):
        #assert(_toybox_json)  // why would we have asserted here? still necessary in the with statement?
        return self.state


if __name__ == "__main__":
  import argparse 

  parser = argparse.ArgumentParser(description='test Amidar interventions')
  parser.add_argument('--partial_config', type=str, default="null")
  parser.add_argument('--save_json', type=bool, default=False)
  args = parser.parse_args()

  with Toybox('breakout') as tb:
    state = tb.to_state_json()
    config = tb.config_to_json()

    if args.save_json:
        # save a sample starting state and config
        with open('toybox/interventions/defaults/breakout_state_default.json', 'w') as outfile:
            json.dump(state, outfile)

        with open('toybox/interventions/defaults/breakout_config_default.json', 'w') as outfile:
            json.dump(config, outfile)
    
    # remove and assert that the brick is gone
    with BreakoutIntervention(tb) as intervention:
        nbricks = intervention.num_bricks_remaining()
        intervention.set_brick(0, alive=False)
        nbricks_post = intervention.num_bricks_remaining()

        assert nbricks - 1 == nbricks_post

    # reset and assert that the brick is present
    with BreakoutIntervention(tb) as intervention:
        nbricks = intervention.num_bricks_remaining()
        intervention.set_brick(0, alive=True)
        nbricks_post = intervention.num_bricks_remaining()

        assert nbricks + 1 == nbricks_post

    # add a channel and assert that num_rows bricks have been removed
    with BreakoutIntervention(tb) as intervention: 
        nbricks = intervention.num_bricks_remaining()

        stacks = intervention.get_stacks()
        intervention.add_channel(stacks[0])
        nbricks_post = intervention.num_bricks_remaining()
        assert nbricks_post == nbricks - intervention.num_rows()

        col, channel = intervention.find_channel()
        assert channel

        assert intervention.channel_count() == 1

    # remove a channel and assert that num_rows bricks have been added
    with BreakoutIntervention(tb) as intervention: 
        nbricks = intervention.num_bricks_remaining()

        stacks = intervention.get_stacks()
        intervention.fill_channel(stacks[0])
        nbricks_post = intervention.num_bricks_remaining()
        assert nbricks_post == nbricks + intervention.num_rows()

    # get ball position, even when multiple balls present
    with BreakoutIntervention(tb) as intervention: 
        ball_pos = intervention.get_ball_position()
        assert ball_pos == {'y': 80.0, 'x':120.0}

        ball_velocity = intervention.get_ball_velocity()
        assert ball_velocity['y'] == 1.0

        intervention.state['balls'] = [intervention.state['balls'][0], intervention.state['balls'][0]]
        ball_positions = intervention.get_ball_position()
        assert len(ball_positions) == 2
        ball_velocities = intervention.get_ball_velocity()
        assert len(ball_velocities) == 2

        intervention.state['balls'] = [intervention.state['balls'][0],]
        ball_positions = intervention.get_ball_position()
        assert ball_positions == {'y': 80.0, 'x':120.0}

    # move ball diagonally by sqrt(2) pixels
    with BreakoutIntervention(tb) as intervention: 
        ball_pos = intervention.get_ball_position()
        ball_pos['x'] = ball_pos['x'] + 1
        ball_pos['y'] = ball_pos['y'] + 1
        intervention.set_ball_position(ball_pos)
        ball_pos_post = intervention.get_ball_position()
        assert ball_pos_post['x'] == ball_pos['x']

        ball_pos['x'] = ball_pos['x'] - 1
        ball_pos['y'] = ball_pos['y'] - 1
        intervention.set_ball_position([ball_pos])
        ball_pos_post = intervention.get_ball_position()
        assert ball_pos_post['x'] == ball_pos['x']


    # change ball velocity
    with BreakoutIntervention(tb) as intervention: 
        ball_vel = intervention.get_ball_velocity()
        ball_vel['x'] = ball_vel['x'] + 1
        ball_vel['y'] = ball_vel['y'] + 1
        intervention.set_ball_velocity(ball_vel)
        ball_vel_post = intervention.get_ball_velocity()
        assert ball_vel_post['x'] == ball_vel['x']

        ball_vel['x'] = ball_vel['x'] - 1
        ball_vel['y'] = ball_vel['y'] - 1
        intervention.set_ball_velocity([ball_vel])
        ball_vel_post = intervention.get_ball_velocity()
        assert ball_vel_post['x'] == ball_vel['x']

    # get paddle position and move
    with BreakoutIntervention(tb) as intervention: 
        pos = intervention.get_paddle_position()
        assert pos['x'] == 120.0 and pos['y'] == 143.0

        pos['x'] = pos['x'] + 10
        intervention.set_paddle_position(pos)
        pos_post = intervention.get_paddle_position()
        assert pos['x'] == pos_post['x']

        pos['x'] = pos['x'] - 10
        intervention.set_paddle_position(pos)
