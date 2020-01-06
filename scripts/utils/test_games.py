import ctoybox
from ctoybox import Toybox, Input
import argparse

def smoke_test(tb):
    print(tb.get_width(), tb.get_height())
    # Score and lives make sense!
    assert(tb.get_score() == 0)
    assert(tb.get_lives() > 0)

    for _ in range(100):
        # NOOP should work for every game.
        tb.apply_action(Input())
    
    # Color rendering should work.
    image = tb.get_rgb_frame()

for game in ['amidar', 'breakout', 'space_invaders']:
    print("TEST ", game)
    with Toybox(game) as tb:
        # config -> json should work:
        config = tb.config_to_json()
        # state -> json should work:
        state = tb.state_to_json()

        actions = tb.get_legal_action_set()
        print("legal_actions", actions)

        # setting a seed should work.
        tb.set_seed(1234)

        # and all sorts of things make sense:
        smoke_test(tb)

        # Writing config should work:
        tb.write_config_json(config)
        smoke_test(tb)

        # Writing state should work:
        tb.write_state_json(state)
        smoke_test(tb)
