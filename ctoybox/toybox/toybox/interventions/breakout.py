from toybox.interventions.base import *
import json
"""An API for interventions on breakout."""

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
        return self.state['bricks'][i]


    def num_bricks(self):
        return len(self.state['bricks'])


    def set_brick(self, index, alive=True):
        self.state['bricks'][index]['alive'] = alive


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
        """Returns a list of lists. Each element returned is a list of indices, corresponding to a potential channel."""
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


# Intervention tests
    # assert number of bricks 
    # assert rows, columns, stacks
    # assert channel present, channel not present 
    # assert can find channel 
    # assert is stack

    # ball position
    # ball speed
    # paddle position
    # paddle speed
    # all the constants!

    # add brick in arbitrary location?


    def get_json(self):
        #assert(_toybox_json)  // why would we have asserted here? still necessary in the with statement?
        return self.state


if __name__ == "__main__":
  with Toybox('breakout') as tb:
    state = tb.to_state_json()
    config = tb.config_to_json()

    # save the 
    with open('toybox/toybox/interventions/defaults/breakout_state_default.json', 'w') as outfile:
        json.dump(state, outfile)

    with open('toybox/toybox/interventions/defaults/breakout_config_default.json', 'w') as outfile:
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

    # remove a channel and assert that num_rows bricks have been added
    with BreakoutIntervention(tb) as intervention: 
        pass


