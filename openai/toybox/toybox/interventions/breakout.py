"""An API for interventions on breakout."""

_toybox = None
_toybox_json = {}

def checktoybox(f):
    def check(toybox, *args, **kwargs):
        global _toybox, _toybox_json
        if toybox and not (_toybox and toybox is _toybox):
            _toybox = toybox
            _toybox_json = toybox.to_json()
        return f(*args, **kwargs)
    return check 


@checktoybox
def get_bricks():
    return _toybox_json['bricks']      


@checktoybox
def get_brick(i):
    return _toybox_json['bricks'][i]


@checktoybox
def num_bricks():
    assert(_toybox_json)
    return len(_toybox_json['bricks'])


@checktoybox
def remove_brick(index):
    assert(_toybox_json)
    _toybox_json['bricks'][index]['alive'] = False


@checktoybox
def add_brick(index):
    _toybox_json['bricks'][index]['alive'] = True


@checktoybox
def find_brick(pred):
    for i, b in enumerate(_toybox_json['bricks']):
        if pred(b):
            return i, b
    raise ValueError('No bricks that satisfy the input predicate found.')


@checktoybox
def num_rows():
    return len(_toybox_json['config']['row_scores'])


@checktoybox
def num_columns():
    rows = num_rows(_toybox)
    bricks = num_bricks(_toybox)
    return bricks // rows


@checktoybox
def get_stacks():
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



@checktoybox
def is_channel(intlist):
    bricks = get_bricks(_toybox)
    return is_stack(intlist) and all([not is_alive(bricks[i]) for i in intlist])


@checktoybox
def find_channel():
    stacks = get_stacks(_toybox)
    for i, stack in enumerate(stacks):
        if is_channel(_toybox, stack):
            return i, stack
    raise ValueError('No channels found.')


@checktoybox
def add_channel(intlist):
    assert(is_stack(intlist))
    for i in intlist:
        remove_brick(_toybox, i)


def is_stack(intlist):
    ncols = num_columns(_toybox)
    nrows = num_rows(_toybox)
    offset = intlist[0] % ncols
    row = 1
    for i in intlist[1:]:
        if i != row * ncols + offset:
            return False
        row += 1
    return True 


def is_alive(brick):
    return brick['alive']


def get_json():
    assert(_toybox_json)
    return _toybox_json