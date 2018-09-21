"""An API for interventions on breakout."""

_toybox = None
_toybox_json = {}

def check_toybox(f):
    def check(toybox, *args, **kwargs):
        global _toybox, _toybox_json
        if not (_toybox and toybox is _toybox):
            _toybox = toybox
            _toybox_json = toybox.to_json()
        return f(*args, **kwargs)
    return check 
      

@check_toybox
def num_bricks():
    assert(_toybox_json)
    return len(_toybox_json['bricks'])


@check_toybox
def remove_brick(index):
    assert(_toybox_json)
    _toybox_json['bricks'][index]['alive'] = False


def get_json():
    assert(_toybox_json)
    return _toybox_json