import toybox_envs
import toybox.interventions.breakout as inter
from toybox import Toybox

def all_but_one(toybox:Toybox):
    # Generate the set of states that have only one brick left
    # Returns a list of toybox instances with different starting states
    # This is generated from an input state where only one brick is left
    index, last_brick = inter.find_brick(toybox, inter.is_alive)
    # Make sure that we are actually using a toybox state where there is only one brick left.
    assert(all([not inter.is_alive(b) for b in inter.get_bricks(tb) if b != last_brick]))
    # Add the input state and force toybox to create a new instance
    tbs = [tb.from_json(tb.to_json())]
    num_bricks = inter.num_bricks(toybox)
    for i in range(num_bricks):
        # if i is the index of the input state, skip
        if i == index:
            continue
        # i is the one brick we want remaining
        inter.add_brick(toybox, i)
        for j in range(num_bricks):
            if j != i:
                inter.remove_brick(toybox, j)
                new_js = inter.get_json()
                tbs.append(tb.from_json(new_js))
    return tbs


def channels(toybox:Toybox):
    index, channel = inter.find_channel(toybox)
    # Make sure that there is only one channel
    assert(all([not inter.is_channel(channel) for i, channel in enumerate(inter.get_stacks())]))
    # Add the input state and force toybox to create a new instance
    original_json = toybox.to_json()
    tbs = [toybox.from_json(original_json)]
    stacks = inter.get_stacks(toybox)
    # Loop through the stacks and add channels
    # If there are already bricks removed in other stacks, that's okay
    for i, stack in enumerate(stacks):
        if i == index:
            continue
        # Prior calls to the loop will affect state, so we should load up a new tb instance 
        inter.add_channel(tb.from_json(original_json), stack)
        new_js = inter.get_json()
        tbs.append(tb.from_json(new_js))
    return tbs


def create_channels(toybox:Toybox):
    # Like a combination of the previous two tests
    # Find the stack where all of the bricks except one are removed
    original_json = toybox.to_json()
    stacks = inter.get_stacks(toybox)
    index, channel = None, None
    
    for i, stack in enumerate(stacks):
        alive = [1 for i in stack if inter.is_alive(inter.get_brick(i))]
        if sum(alive) == 1:
            index, channel = i, stack
            break
    
    assert(not (index and channel))
    
    tbs = []
    for i, stack in enumerate(stacks):
        for j in stack:
            # j will be the "last brick"
            this_tb = tb.from_json(tb.to_json(toybox))
            inter.add_brick(this_tb, j)
            for k in stack:
                if j == k:
                    continue
                inter.remove_brick(this_tb, k)
            new_js = inter.get_json()
            tbs.append(tb.from_json(new_js))

    return tbs
        

if __name__ == '__main__':
    with Toybox('breakout') as tb:
        print(len(all_but_one(tb)))
        print(len(channels(tb)))
        print(len(create_channels(tb)))
