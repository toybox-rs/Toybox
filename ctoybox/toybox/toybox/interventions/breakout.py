"""An API for interventions on breakout."""

class BreakoutInterventions(Intervention):

    def breakout_brick_live_by_index(self, index):
        return self.tb.rstate.query_json('brick_live_by_index', json.dumps(index))

    def breakout_bricks_remaining(self):
        return self.tb.rstate.query_json('bricks_remaining')
    
    def breakout_channel_count(self):
        return self.tb.rstate.query_json('count_channels')
    
    def breakout_num_columns(self):
        return self.tb.rstate.query_json('num_columns')

    def breakout_num_rows(self):
        return self.tb.rstate.query_json('num_rows')

    def breakout_channels(self):
        return self.tb.rstate.query_json('channels')


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
        rows = num_rows(_toybox)
        bricks = num_bricks(_toybox)
        return bricks // rows


    def get_stacks(self):
        """Returns a list of lists. Each element returned is a list of indices, corresponding to a potential channel."""
        ncols = num_columns(_toybox)
        nrows = num_rows(_toybox)
        stacks = []
        for offset in range(ncols):
            stack = []
            for row in range(nrows):
                stack.append(row * ncols + offset)
            stacks.append(stack)
        return stacks



    def is_channel(self, intlist):
        bricks = get_bricks(_toybox)
        return is_stack(intlist) and all([not is_alive(bricks[i]) for i in intlist])


    def find_channel(self):
        stacks = get_stacks(_toybox)
        for i, stack in enumerate(stacks):
            if is_channel(_toybox, stack):
                return i, stack
        raise ValueError('No channels found.')


    def add_channel(self, intlist):
        assert(is_stack(intlist))
        for i in intlist:
            remove_brick(_toybox, i)


    def is_stack(self, intlist):
        ncols = num_columns(_toybox)
        nrows = num_rows(_toybox)
        offset = intlist[0] % ncols
        row = 1
        for i in intlist[1:]:
            if i != row * ncols + offset:
                return False
            row += 1
        return True 


# Intervention tests
    # remove and assert that the brick is gone
    # reset and assert that the brick is present
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


    def get_json(self):
        assert(_toybox_json)
        return _toybox_json