from toybox.interventions.base import *
import random
import json
"""An API for interventions on Amidar."""

class AmidarIntervention(Intervention):

    def __init__(self, tb, game_name='amidar'):
        # check that the simulation in tb matches the game name.
        Intervention.__init__(self, tb, game_name)

    def check_position(pdict, ls): 
        # check that pdict is a dictionary containing the keys in list ls
        assert isinstance(pdict, dict)
        assert all([k in pdict.keys() for k in ls])

    def check_tile_position(tdict): 
        assert check_position(tdict, ['tx', 'ty'])
        # check that the tile is a non-empty tile (i.e., is paintable and walkable)

    def num_tiles_unpainted(self):
        total_unpainted = 0
        for i in range(len(self.state['board'])): 
            total_unpainted += sum([tile == "Unpainted" for tile in self.state['board'][i]])

        return total_unpainted

        
    def player_tile(self):
        return self.state['player']['position']

    def num_enemies(self):
        return len(self.state['enemies'])

    def jumps_remaining(self):
        return self.state['jumps']

    def regular_mode(self):
        return self.state['jump_timer'] == 0 and self.state['chase_timer'] == 0

    def jump_mode(self):
        return self.state['jump_timer'] > 0

    def chase_mode(self):
        return self.state['chase_timer'] > 0

    def enemy_tiles(self):
        return [self.state['enemies'][i]['position'] for i in range(len(self.state['enemies']))]

    def enemy_caught(self, eid):
        return self.state['enemies'][eid]['caught']

    def any_enemy_caught(self, eid):
        return any([self.state['enemies'][i]['caught'] for i in range(len(self.state['enemies']))])


    def set_tile(self, tid, paint=True):
        tiles = self.state['board']['tiles']
        assert tiles[tid['tx']][tid['ty']] != "Empty"
        assert check_position(tid, ['tx', 'ty'])

        label = "Painted" if paint else "Unpainted"
        self.state['board']['tiles'][tid['tx']][tid['ty']] = label


    def set_box(self, bid, paint=True, include_tiles=True, allow_chase=True): 
        box = self.state['board']['boxes'][bid]
        box['painted'] = paint

        if allow_chase: 
            # if we allow the intervention to trigger chasing, 
            # we only want to do so if it was not already triggered 
            allow_chase = allow_chase and not self.check_chase_condition()

        if include_tiles: 
            tx_left = box['top_left']['tx']
            ty_left = box['top_left']['ty']
            tx_right = box['bottom_right']['tx']
            ty_right = box['bottom_right']['ty']

            for i in range(tx_left, tx_right+1): 
                for j in range(ty_left, ty_right+1):
                    if self.state['board']['tiles'][i][j] != "Empty": 
                        self.state['board']['tiles'][i][j] = "Painted"

        if allow_chase: 
            self.chase_react()


    def check_chase_condition(self): 
        chase = False
        continue_check = True

        for box in self.state['board']['boxes']: 
            if box['triggers_chase']: 
                tx_left = box['top_left']['tx']
                ty_left = box['top_left']['ty']
                tx_right = box['bottom_right']['tx']
                ty_right = box['bottom_right']['ty']

                all_painted = True
                for i in range(tx_left, tx_right+1): 
                    for j in range(ty_left, ty_right+1):
                        if self.state['board']['tiles'][i][j] != "Empty": 
                            all_painted &= self.state['board']['tiles'][i][j] == "Painted"
                        if not all_painted: 
                            continue_check = False
                            break
                    if not continue_check:
                        break

            if not continue_check: 
                break

        return all_painted


    def chase_react(self): 
        if self.check_chase_condition(): 
            self.set_mode('chase')


    def set_player_tile(self, pos):
        assert check_position(pos, ['x','y'])

        self.state['player']['position']['x'] = pos['x']
        self.state['player']['position']['y'] = pos['y']


    def set_enemy_tile(self, eid, pos):
        assert check_position(pos, ['x', 'y'])

        self.state['enemies'][eid]['position']['x'] = pos['x']
        self.state['enemies'][eid]['position']['y'] = pos['y']


    def set_enemy_tiles(self, eids, pos_list):
        assert len(eids) == len(pos_list)

        for i, eid in enumerate(eids): 
            set_enemy_tile(eid, pos_list[i])


    def set_mode(self, mode_id='regular', set_time=None): 
        assert mode_id in ['jump', 'chase', 'regular']

        if mode_id == 'jump': 
            self.state['jump_timer'] = self.config['jump_time'] if set_time is None else set_time
        elif mode_id == 'chase': 
            self.state['chase_timer'] = self.config['chase_time'] if set_time is None else set_time
        else: #mode_id == 'regular' 
            self.state['jump_timer'] = 0
            self.state['chase_timer'] = 0


    def get_enemy_protocol(self, eid): 
        return self.state['enemies'][eid]['ai']


    def get_enemy_protocols(self, eids): 
        return [self.state['enemies'][eid]['ai'] for eid in eids]


    def set_enemy_protocol(self, eid, protocol='EnemyAmidarMvmt', **kwargs):
        enemy = self.state['enemies'][eid]
        
        enemy['ai'] = {}
        enemy['ai'][protocol] = get_default_protocol(protocol, kwargs)


    def get_default_protocol(self, protocol, e_pos, **kwarg): 
        assert protocol in ['EnemyLookupAI', 'EnemyPerimeterAI', 'EnemyAmidarMvmt', 'EnemyTargetPlayer', 'EnemyRandomMvmt']
        protocol_ai = {}

        if protocol == 'EnemyLookupAI': 
            protocol_ai['default_route_index'] = eid % 5 if 'default_route_index' not in kwargs.keys() else kwargs['default_route_index']
            protocol_ai['next'] = 0 if 'next' not in kwargs.keys() else kwargs['next']

        # add start position
        if protocol in ['EnemyPerimeterAI', 'EnemyRandomMvmt', 'EnemyTargetPlayer', 'EnemyAmidarMvmt']: 
            if 'start' in kwargs.keys(): 
                assert check_position(kwargs['start'], ['tx', 'ty'])
                protocol_ai['start'] = kwargs['start']
            else:
                protocol_ai['start'] = get_random_position()

        # add current direction and start direction
        if protocol in ['EnemyRandomMvmt', 'EnemyTargetPlayer']:
            # choose a valid starting direction 
            protocol_ai['start_dir'] = get_random_dir_for_tile(protocol_ai['start']) if 'start_dir' not in kwargs.keys() else kwargs['start_dir']

            # choose a valid direction to move in
            assert check_position(e_pos, ['tx']['ty'])
            protocol_ai['dir'] = get_random_dir_for_tile(e_pos) if 'dir' not in kwargs.keys() else kwargs['dir']

        if protocol == 'EnemyTargetPlayer':
            protocol_ai['vision_distance'] = 15 if 'vision_distance' not in kwargs.keys() else kwargs['vision_distance']

            # should have some (Rust?) check to see if enemies move toward the player on the first step after changing protocol
            # for now, just assume the first step after setting is False
            protocol_ai['player_seen'] = False if 'player_seen' not in kwargs.keys() else kwargs['player_seen']

        if protocol == 'EnemyAmidarMvmt':
            protocol_ai['vert'] = random.choice(["Up", "Down"]) if 'vert' not in kwargs.keys() else kwargs['vert']
            protocol_ai['horiz']= random.choice(["Left", "Right"]) if 'horiz' not in kwargs.keys() else kwargs['horiz']
            protocol_ai['start_vert'] = random.choice(["Up", "Down"]) if 'start_vert' not in kwargs.keys() else kwargs['start_vert']
            protocol_ai['start_horiz'] = random.choice(["Up", "Down"]) if 'start_horiz' not in kwargs.keys() else kwargs['start_horiz']
           

        return protocol_ai

    def get_random_tile_id(self): 
        tile = {}

        tx = random.choice(range(len(self.state['board']['tiles'])))
        which_tiles = [i for i in range(len(self.state['board']['tiles'][tx])) if self.state['board']['tiles'][tx][i] != "Empty"]
        ty = random.chioce(which_tiles)

        tile['tx'] = tx
        tile['ty'] = ty

        return tile


    def get_random_position(self):
        rand_tile = self.get_random_tile_id()

        # convert random tile to x,y location 
        pos = {}

        return pos


    def get_random_dir_for_tile(self, tid):
        assert check_position(tid, ['tx', 'ty']) 
        tile = self.state['board']['tiles'][tid['tx']][tid['ty']]

        assert tile != "Empty"
        selected = False
        dirs = ["Up", "Down", "Left", "Right"]

        d = None
        while not selected: 
            next_tid = {}
            next_tid['tx'] = tid['tx']
            next_tid['ty'] = tid['ty']

            if d is not None: 
                dirs.remove(d)
                if dirs.empty(): 
                    d = None

            d = random.choice(dirs)
            if d == "Up":
                next_tid['ty'] = next_tid['ty'] - 1
            elif d == "Down": 
                next_tid['ty'] = next_tid['ty'] + 1
            elif d == "Left": 
                next_tid['tx'] = next_tid['tx'] - 1
            else: # d == "Right"
                next_tid['tx'] = next_tid['tx'] + 1

            selected = not selected and check_tile_position(next_tid, ['tx', 'ty'])

        if d is None:
            raise Exception("No valid direction from this tile:\t\tTile tx:"+str(tid['tx'])+", ty"+str(tid['ty']))

        return d


    
    def set_enemy_protocols(self, eids, protocols=None):
        if protocols is None: 
            protocols = ['EnemyAmidarMvmt']*len(eids) 
        assert len(eids) == len(protocols)

        for i, eid in enumerate(eids):
            self.set_enemy_protocol[eid, protocols[i]] 


    def add_enemy(self, pos, ai='EnemyLookupAI', **kwargs): 
        new_e = {}

        # append kwargs, fill in with defaults
        new_e['history'] = [] if not 'history' in kwargs.keys() else kwargs['history']
        new_e['step'] = None if not 'step' in kwargs.keys() else kwargs['step']
        new_e['caught'] = False if not 'caught' in kwargs.keys() else kwargs['caught']
        new_e['speed'] = 8 if not 'speed' in kwargs.keys() else kwargs['speed']

        assert check_position(pos, ['x', 'y'])
        new_e['position'] = pos
        self.state['enemies'].append(new_e)        

        self.set_enemy_protocol(-1, ai, kwargs)        


    def remove_enemy(self, eid):
        eids = range(len(self.state['enemies']))

        assert eid in eids
        eids.remove(eid)

        self.state['enemies'] = self.state['enemies'][eids]


    def set_n_jumps(self, n):  
        assert n >= 0
        self.state['jumps'] = n


    def set_n_lives(self, n):
        assert n > 0 
        self.state['lives'] = n

    # set enemy protocol 
    # get, set score
        # consider logic for score calculation

### difficult interventions ###
    # random start state?
    # enemy perimeter direction 
    # tie random selections to Toybox environment seed?


if __name__ == "__main__":
    import argparse 

    parser = argparse.ArgumentParser(description='test Amidar interventions')
    parser.add_argument('--partial_config', type=str, default="null")
    parser.add_argument('--save_json', type=bool, default=False)

    args = parser.parse_args()

    with Toybox('amidar') as tb:
        state = tb.to_state_json()
        config = tb.config_to_json()

    if args.save_json:
        # save a sample starting state and config
        with open('toybox/toybox/interventions/defaults/amidar_state_default.json', 'w') as outfile:
            json.dump(state, outfile)

        with open('toybox/toybox/interventions/defaults/amidar_config_default.json', 'w') as outfile:
            json.dump(config, outfile)
