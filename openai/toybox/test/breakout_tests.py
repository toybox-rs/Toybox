import toybox.interventions.breakout as breakout
from toybox.toybox import Toybox

def all_but_one(toybox):
    # Generate the set of states that have only one brick left
    # Returns a list of toybox instances with different starting states
    json = toybox.to_json()
    tbs = []
    num_bricks = breakout.num_bricks(toybox)
    assert(isinstance(num_bricks, int))
    for i in range(num_bricks):
        # i is the one brick we want remaining
        for j in range(num_bricks):
            if j is not i:
                breakout.remove_brick(toybox, j)
                new_js = breakout.get_json()
                tbs.append(toybox.from_json(new_js))
    return tbs


if __name__ == '__main__':
    with Toybox('breakout') as tb:
        print(len(all_but_one(tb)))