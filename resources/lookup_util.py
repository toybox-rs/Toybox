import numpy as np
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
    return tilepoint_lookup(intervention, target_tile)

def tile_to_route_id(intervention, tx, ty):
    return intervention.game.board.width*ty+tx

def tilepoint_lookup(intervention, target_tile_id):
    ty = target_tile_id // intervention.game.board.width
    tx = target_tile_id - ty * intervention.game.board.width
    return ami.TilePoint(intervention, tx, ty)

def _get_junction_adj_mat(intervention):
    b = intervention.game.board
    njunctions = len(b.junctions)
    junction_adj = np.zeros((njunctions, njunctions), int)
    js = np.asarray(b.junctions)
    # fill in horizontal adj junctions
    for row in b.tiles:
        prev_j = None
        for tile in row:
            tp = intervention.tile_to_tilepoint(tile)
            rid = tile_to_route_id(intervention, tp.tx, tp.ty)
            if rid in b.junctions:
                j_id = np.argwhere(js == rid)[0][0]
                if prev_j is not None:
                    junction_adj[prev_j, j_id] = 1
                    junction_adj[j_id, prev_j] = 1
                prev_j = j_id
    # fill in vertical adj junctions
    vert_iterable = [[row[i] for row in b.tiles] for i, e in enumerate(b.tiles[0])]
    for col in vert_iterable:
        prev_j = None
        for tile in col:
            tp = intervention.tile_to_tilepoint(tile)
            rid = tile_to_route_id(intervention, tp.tx, tp.ty)
            if rid in b.junctions:
                j_id = np.argwhere(js == rid)[0][0]
                if prev_j is not None:
                    junction_adj[prev_j, j_id] = 1
                    junction_adj[j_id, prev_j] = 1
                prev_j = j_id
    return junction_adj

def segment_lookup(intervention):
    # iterate over all tiles in intervention.game.board to identify adjacent junctions
    # use junction id tuples to define segments
    junction_adj = _get_junction_adj_mat(intervention)
    junctions = intervention.game.board.junctions

    # identify all segments with junction endpoints
    adj_jns = np.argwhere(junction_adj == 1)
    # segments.append((junction_id1, junction_id2))
    segments = []
    for jn_endpoints in adj_jns:
        # identify segments from endpoints
        jid1, jid2 = jn_endpoints
        rid1 = junctions[jid1]
        rid2 = junctions[jid2]
        seg_id =  str(rid1) + '.' + str(rid2)
        segments.append(seg_id)

    # segment to junctions lookup
    segment_junction_lookup = {}
    for seg in segments:
        t1, t2 = [tilepoint_lookup(intervention, int(s)) for s in seg.split('.')]
        segment_junction_lookup[seg] = [tile_to_route_id(intervention, t1.tx, t1.ty),
                                        tile_to_route_id(intervention, t2.tx, t2.ty)]

    # junction to adj junctions lookup
    junction_adjacency_lookup = {}
    for j in junctions:
        junction_adjacency_lookup[j] = set()
    for i, jn in enumerate(junctions):
        # add adjacent junctions to dict
        # add as undirected
        adj_nda = np.argwhere(junction_adj[i,] == 1)
        adj_idx = [list(adj_nda[i])[0] for i in range(len(adj_nda))]
        adj_jids = [junctions[i] for i in adj_idx]
        for jid in adj_jids:
            junction_adjacency_lookup[jn].add(jid)
            junction_adjacency_lookup[jid].add(jn)
    for k in junction_adjacency_lookup.keys():
        junction_adjacency_lookup[k] = list(junction_adjacency_lookup[k])

    # tile to segment lookup
    tilepoint_segment_lookup = {}
    for seg in segments:
        t1, t2 = [tilepoint_lookup(intervention, int(s)) for s in seg.split('.')]
        for tx in range(t1.tx, t2.tx+1):
            for ty in range(t1.ty, t2.ty+1):
                rid = tile_to_route_id(intervention, tx, ty)
                tilepoint_segment_lookup[rid] = seg

    return segments, segment_junction_lookup, junction_adjacency_lookup, tilepoint_segment_lookup


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




