import toybox.interventions.amidar as ami

def read_lookup():
    try:
        with open("resources/amidar_enemy_positions") as f:
            tile_lists = f.readlines()
            lookup_table = [e.split(' ') for e in tile_lists]

        return lookup_table

    except: # the ol' catch and release
        return None

def route_step_to_tile(intervention, route_index, route_step, lookup_table=None):
    if lookup_table is None:
        lookup_table = read_lookup()

    route_step = route_step % len(lookup_table[route_index])
    target_tile = int(lookup_table[route_index][route_step])
    return tilepoint_lookup(intervention, target_tile, lookup_table=None)

def tile_to_route_id(intervention, tx, ty):
    return intervention.game.board.width*ty+tx

def tilepoint_lookup(intervention, target_tile_id, lookup_table=None):
    if lookup_table is None:
        lookup_table = read_lookup()

    ty = target_tile_id // intervention.game.board.width
    tx = target_tile_id - ty * intervention.game.board.width
    return ami.TilePoint(intervention, tx, ty)

def shift_enemy_defaults(intervention, shift_vector, lookup_table=None):
    if lookup_table is None:
        lookup_table = read_lookup()
    for i, e in enumerate(intervention.game.enemies):
        # assign shift amount
        e.ai.next = (e.ai.next + shift_vector[i]) % len(lookup_table[e.ai.default_route_index])
        # set the enemy position to the offset lookup tile position
        target_tile = route_step_to_tile(intervention, e.ai.default_route_index, e.ai.next, lookup_table)
        e.step = target_tile
        e.position = intervention.tilepoint_to_worldpoint(e.step)

def discover_tile_hardness():
    # to run from cmd line:
    # ./start_python -c 'from resources.lookup_util import discover_tile_hardness; discover_tile_hardness()'
    from toybox.sample_tests.test_util_amidar import AmidarCrawler
    import unittest

    tsuite = unittest.TestSuite()
    tsuite.addTest(AmidarCrawler('simulate'))

    runner = unittest.TextTestRunner()
    runner.run(tsuite)




