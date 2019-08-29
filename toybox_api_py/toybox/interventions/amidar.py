from toybox.interventions.base import *
import random
import json
"""An API for interventions on Amidar."""

mvmt_protocols = ['EnemyLookupAI', 'EnemyPerimeterAI', 'EnemyAmidarMvmt', 'EnemyTargetPlayer', 'EnemyRandomMvmt']

class AmidarIntervention(Intervention):

    def __init__(self, tb, game_name='amidar'):
        # check that the simulation in tb matches the game name.
        Intervention.__init__(self, tb, game_name)

    ### helper functions ###
    def check_tile_position(self, tdict):
        # check that the tile is a non-empty tile (i.e., is paintable and walkable)
        walkable = self.check_is_tile(tdict)
        # assertion will stop the script if not met; different behavior from check_is_tile
        assert walkable
        return True


    def check_is_tile(self, tile_pos): 
        assert Intervention.check_position(self, tile_pos, ['tx', 'ty'])
        # check the bounds of the position tile
        in_bounds = tile_pos['ty'] >= 0 and tile_pos['ty'] < len(self.state['board']['tiles'])
        in_bounds = in_bounds and (tile_pos['tx'] >= 0 and tile_pos['tx'] < len(self.state['board']['tiles'][tile_pos['ty']]))
        if not in_bounds: 
            return False
        # make sure tile is walkable
        walkable = self.state['board']['tiles'][tile_pos['ty']][tile_pos['tx']] != "Empty"
        return in_bounds and walkable

    def get_random_tile_id(self): 
        tile = {}

        ty = random.choice(range(len(self.state['board']['tiles'])))
        which_tiles = [i for i in range(len(self.state['board']['tiles'][ty])) if self.state['board']['tiles'][ty][i] != "Empty"]
        tx = random.choice(which_tiles)

        tile['tx'] = tx
        tile['ty'] = ty

        return tile


    def get_random_position(self):
        # grab random tile
        rand_tile = self.get_random_tile_id()
    
        # convert random tile to x,y location 
        x, y = self.tile_to_world(rand_tile['tx'], rand_tile['ty'])

        # return in dictionary
        pos = {}
        pos['x'] = x
        pos['y'] = y

        return pos


    def world_to_tile(self, x, y): 
        worldpoint = {'x': x, 'y': y}
        (tx, ty) = self.toybox.query_state_json("world_to_tile", worldpoint)

        tile = {}
        tile['tx'] = tx
        tile['ty'] = ty
        return tile


    def tile_to_world(self, tx, ty): 
        tilepoint = {'tx': tx, 'ty': ty}
        (x, y) = self.toybox.query_state_json("tile_to_world", tilepoint)

        pos = {}
        pos['x'] = x
        pos['y'] = y
        return pos


    def num_tiles_unpainted(self):
        total_unpainted = 0
        for i in range(len(self.state['board']['tiles'])): 
            total_unpainted += sum([tile != "Painted" and tile != "Empty" for tile in self.state['board']['tiles'][i]])

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


    def set_tile_paint(self, tid, paint=True):
        tiles = self.state['board']['tiles']
        assert self.check_is_tile(tid)

        label = "Painted" if paint else "Unpainted"
        self.state['board']['tiles'][tid['ty']][tid['tx']] = label


    # def set_box(self, bid, paint=True, include_tiles=True, allow_chase=True): 
    #     box = self.state['board']['boxes'][bid]
    #     box['painted'] = paint

    #     if allow_chase: 
    #         # if we allow the intervention to trigger chasing, 
    #         # we only want to do so if it was not already triggered 
    #         allow_chase = allow_chase and not self.check_chase_condition()

    #     if include_tiles: 
    #         tx_left = box['top_left']['tx']
    #         ty_left = box['top_left']['ty']
    #         tx_right = box['bottom_right']['tx']
    #         ty_right = box['bottom_right']['ty']

    #         for i in range(tx_left, tx_right+1): 
    #             for j in range(ty_left, ty_right+1):
    #                 if self.state['board']['tiles'][i][j] != "Empty": 
    #                     self.state['board']['tiles'][i][j] = "Painted"

    #     if allow_chase: 
    #         self.chase_react()


    # def check_chase_condition(self): 
    #     chase = False
    #     continue_check = True

    #     for box in self.state['board']['boxes']: 
    #         if box['triggers_chase']: 
    #             tx_left = box['top_left']['tx']
    #             ty_left = box['top_left']['ty']
    #             tx_right = box['bottom_right']['tx']
    #             ty_right = box['bottom_right']['ty']

    #             all_painted = True
    #             for i in range(tx_left, tx_right+1): 
    #                 for j in range(ty_left, ty_right+1):
    #                     if self.state['board']['tiles'][i][j] != "Empty": 
    #                         all_painted &= self.state['board']['tiles'][i][j] == "Painted"
    #                     if not all_painted: 
    #                         continue_check = False
    #                         break
    #                 if not continue_check:
    #                     break

    #         if not continue_check: 
    #             break

    #     return all_painted


    # def chase_react(self): 
    #     if self.check_chase_condition(): 
    #         self.set_mode('chase')


    # def set_player_tile(self, pos):
    #     # check the input formatting
    #     assert Intervention.check_position(self, pos, ['x','y'])

    #     # get tile for new position
    #     # check that this is a walkable, valid tile
    #     tile = self.world_to_tile(pos)
    #     assert self.check_tile_position(tile)

    #     # now set the position
    #     self.state['player']['position']['x'] = pos['x']
    #     self.state['player']['position']['y'] = pos['y']


    # def set_enemy_tile(self, eid, pos):
    #     assert Intervention.check_position(self, pos, ['x', 'y'])

    #     self.state['enemies'][eid]['position']['x'] = pos['x']
    #     self.state['enemies'][eid]['position']['y'] = pos['y']


    # def set_enemy_tiles(self, eids, pos_list):
    #     assert len(eids) == len(pos_list)

    #     for i, eid in enumerate(eids): 
    #         set_enemy_tile(eid, pos_list[i])


    def set_mode(self, mode_id='regular', set_time=None): 
        assert mode_id in ['jump', 'chase', 'regular']

        if mode_id == 'jump': 
            self.state['jump_timer'] = self.config['jump_time'] if set_time is None else set_time
        elif mode_id == 'chase': 
            self.state['chase_timer'] = self.config['chase_time'] if set_time is None else set_time
        else: #mode_id == 'regular' 
            self.state['jump_timer'] = 0
            self.state['chase_timer'] = 0


    # def get_enemy_protocol(self, eid): 
    #     return self.state['enemies'][eid]['ai']


    # def get_enemy_protocols(self, eids): 
    #     return [self.state['enemies'][eid]['ai'] for eid in eids]


    def set_enemy_protocol(self, eid, protocol='EnemyAmidarMvmt', **kwargs):
        assert 'ai' in self.state['enemies'][eid].keys()

        new_protocol = {}
        new_protocol[protocol] = self.get_default_protocol(protocol, eid, self.state['enemies'][eid]['position'], **kwargs)
        self.state['enemies'][eid]['ai'] = new_protocol


    def get_default_protocol(self, protocol, eid, e_pos, **kwargs): 
        assert protocol in mvmt_protocols
        protocol_ai = {}

        if protocol == 'EnemyLookupAI': 
            protocol_ai['default_route_index'] = eid % 5 if 'default_route_index' not in kwargs.keys() else kwargs['default_route_index']
            protocol_ai['next'] = 0 if 'next' not in kwargs.keys() else kwargs['next']

        # add start position
        if protocol in ['EnemyPerimeterAI', 'EnemyRandomMvmt', 'EnemyTargetPlayer', 'EnemyAmidarMvmt']: 
            if 'start' in kwargs.keys(): 
                assert Intervention.check_position(self, kwargs['start'], ['tx', 'ty'])
                protocol_ai['start'] = kwargs['start']
            else:
                protocol_ai['start'] = self.get_random_tile_id()

        # add current direction and start direction
        if protocol in ['EnemyRandomMvmt', 'EnemyTargetPlayer']:
            # choose a valid starting direction 
            protocol_ai['start_dir'] = self.get_random_dir_for_tile(protocol_ai['start']) if 'start_dir' not in kwargs.keys() else kwargs['start_dir']

            # choose a valid direction to move in
            e_tile = self.world_to_tile(e_pos['x'], e_pos['y'])
            assert self.check_tile_position(e_tile)
            protocol_ai['dir'] = self.get_random_dir_for_tile(e_tile) if 'dir' not in kwargs.keys() else kwargs['dir']

        if protocol == 'EnemyTargetPlayer':
            protocol_ai['vision_distance'] = 15 if 'vision_distance' not in kwargs.keys() else kwargs['vision_distance']

            # should have some (Rust?) check to see if enemies move toward the player on the first step after changing protocol
            # for now, just assume for the first step after setting that 'player_seen' is None
            protocol_ai['player_seen'] = None if 'player_seen' not in kwargs.keys() else kwargs['player_seen']

        if protocol == 'EnemyAmidarMvmt':
            protocol_ai['vert'] = random.choice(["Up", "Down"]) if 'vert' not in kwargs.keys() else kwargs['vert']
            protocol_ai['horiz']= random.choice(["Left", "Right"]) if 'horiz' not in kwargs.keys() else kwargs['horiz']
            protocol_ai['start_vert'] = random.choice(["Up", "Down"]) if 'start_vert' not in kwargs.keys() else kwargs['start_vert']
            protocol_ai['start_horiz'] = random.choice(["Up", "Down"]) if 'start_horiz' not in kwargs.keys() else kwargs['start_horiz']

        return protocol_ai


    def get_random_dir_for_tile(self, tid):
        assert self.check_tile_position(tid) 
        tile = self.state['board']['tiles'][tid['ty']][tid['tx']]

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
                if not dirs: 
                    d = None

            d = random.choice(dirs)
            if d == "Up":
                next_tid['ty'] = next_tid['ty'] - 1
            elif d == "Down": 
                next_tid['ty'] = next_tid['ty'] + 1
            elif d == "Left": 
                next_tid['tx'] = next_tid['tx'] - 1
            elif d == "Right":
                next_tid['tx'] = next_tid['tx'] + 1

            if d is not None: 
                selected = not selected and self.check_is_tile(next_tid)

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

        assert Intervention.check_position(self, pos, ['x', 'y'])
        new_e['position'] = pos
        
        # we can't use self.set_enemy_protocol(...) here because Rust will rightly complain that the enemy added to 
        # the list of enemies does not contain a movement protocol

        # instead, we add the default protocol JSON to the new enemy
        eid = self.num_enemies()
        new_e['ai'] = {}
        new_e['ai'][ai] = self.get_default_protocol(ai, eid, pos, **kwargs)

        # then append the complete enemy to the list
        self.state['enemies'].append(new_e)


    def remove_enemy(self, eid):
        assert eid < self.num_enemies() and eid >=0 
        enemies = [self.state['enemies'][i] for i in range(len(self.state['enemies'])) if i != eid]
        self.state['enemies'] = enemies


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
    parser.add_argument('--save_tile_images', type=bool, default=False)

    pre_img_path = "pre_img.jpg"
    post_img_path = "post_img.jpg"

    args = parser.parse_args()

    with Toybox('amidar') as tb:
        state = tb.to_state_json()
        config = tb.config_to_json()

        if args.save_json:
            # save a sample starting state and config
            with open('toybox/interventions/defaults/amidar_state_default.json', 'w') as outfile:
                json.dump(state, outfile)

            with open('toybox/interventions/defaults/amidar_config_default.json', 'w') as outfile:
                json.dump(config, outfile)

         # num tiles unpainted
        with AmidarIntervention(tb) as intervention:
            tiles_unpainted = intervention.num_tiles_unpainted()
        #assert tiles_unpainted == 356

        if args.save_tile_images: 
            for ty in range(len(state['board']['tiles'])):
                for tx in range(len(state['board']['tiles'][ty])):
                    tile_pos = {'tx':tx, 'ty':ty}

                    with AmidarIntervention(tb) as intervention:
                        is_tile = intervention.check_is_tile(tile_pos)

                    if is_tile:
                        with AmidarIntervention(tb) as intervention:
                                intervention.set_tile_paint(tile_pos)

                        fname = 'tile_tx_'+str(tx)+'_ty_'+str(ty)+'.jpg'
                        tb.save_frame_image(fname, grayscale=False)

                        with AmidarIntervention(tb) as intervention: 
                            intervention.set_tile_paint(tile_pos, False)

       

        # test painting
        tile_pos = {'tx':0, 'ty':0}
        with AmidarIntervention(tb) as intervention:
            intervention.set_tile_paint(tile_pos)
        with AmidarIntervention(tb) as intervention:
            assert intervention.state['board']['tiles'][tile_pos['ty']][tile_pos['tx']] == "Painted"


        # test unpainting
        tile_pos = {'tx':0, 'ty':0}
        with AmidarIntervention(tb) as intervention:
            intervention.set_tile_paint(tile_pos, False)
        with AmidarIntervention(tb) as intervention:
            # note that this tile was originally a ChaseMarker but that case is not handled 
            # by these intervention functions
            assert intervention.state['board']['tiles'][tile_pos['ty']][tile_pos['tx']] == "Unpainted"


        # get number of enemies
        with AmidarIntervention(tb) as intervention: 
            n_enemies = intervention.num_enemies()
        assert n_enemies == 5


        # remove enemy
        with AmidarIntervention(tb) as intervention: 
            pos = intervention.state['enemies'][4]['position']
            intervention.remove_enemy(4)
        # get number of enemies
        with AmidarIntervention(tb) as intervention: 
            n_enemies = intervention.num_enemies()
        # check one has been removed
        assert n_enemies == 4


        # add enemy with 'EnemyLookupAI' protocol
        with AmidarIntervention(tb) as intervention: 
            #pos = get_random_position()
            intervention.add_enemy(pos, speed=8)
        # get number of enemies
        with AmidarIntervention(tb) as intervention: 
            n_enemies = intervention.num_enemies()
        # check one has been added back
        assert n_enemies == 5

        # change to 'EnemyPerimeterAI' protocol
        change_eid = n_enemies - 1
        with AmidarIntervention(tb) as intervention: 
            intervention.set_enemy_protocol(change_eid, 'EnemyPerimeterAI')
        with AmidarIntervention(tb) as intervention: 
            ai_keys = intervention.state['enemies'][change_eid]['ai'].keys()
            ai_args = intervention.state['enemies'][change_eid]['ai']['EnemyPerimeterAI'].keys()
        assert 'EnemyPerimeterAI' in ai_keys and len(ai_keys) == 1 
        print('EnemyPerimeterAI', ai_args)
        #assert len(ai_args) == 5

        # change to 'EnemyAmidarMvmt' protocol
        with AmidarIntervention(tb) as intervention: 
            intervention.set_enemy_protocol(change_eid, 'EnemyAmidarMvmt')
        with AmidarIntervention(tb) as intervention: 
            ai_keys = intervention.state['enemies'][change_eid]['ai'].keys()
            ai_args = intervention.state['enemies'][change_eid]['ai']['EnemyAmidarMvmt'].keys()
        assert 'EnemyAmidarMvmt' in ai_keys and len(ai_keys) == 1 
        print('EnemyAmidarMvmt', ai_args)
        assert len(ai_args) == 5

        # change to 'EnemyTargetPlayer' protocol
        with AmidarIntervention(tb) as intervention: 
            intervention.set_enemy_protocol(change_eid, 'EnemyTargetPlayer')
        with AmidarIntervention(tb) as intervention: 
            ai_keys = intervention.state['enemies'][change_eid]['ai'].keys()
            ai_args = intervention.state['enemies'][change_eid]['ai']['EnemyTargetPlayer'].keys()
        assert 'EnemyTargetPlayer' in ai_keys and len(ai_keys) == 1 
        print('EnemyTargetPlayer', ai_args)


        # change to 'EnemyRandomAI' protocol
        with AmidarIntervention(tb) as intervention: 
            intervention.set_enemy_protocol(change_eid, 'EnemyRandomMvmt')
        with AmidarIntervention(tb) as intervention: 
            ai_keys = intervention.state['enemies'][change_eid]['ai'].keys()
            ai_args = intervention.state['enemies'][change_eid]['ai']['EnemyRandomMvmt'].keys()
        assert 'EnemyRandomMvmt' in ai_keys and len(ai_keys) == 1 
        print('EnemyRandomMvmt', ai_args)



        # check number of jumps
        with AmidarIntervention(tb) as intervention: 
            n_jumps = intervention.jumps_remaining()
        assert n_jumps == 4

        # set number of jumps
        with AmidarIntervention(tb) as intervention:
            intervention.set_n_jumps(5)
        with AmidarIntervention(tb) as intervention:
            n_jumps = intervention.jumps_remaining()
        assert n_jumps == 5


        # check number of lives
        # set number of lives

        # check jump mode
        with AmidarIntervention(tb) as intervention:
            intervention.set_mode('jump')
        with AmidarIntervention(tb) as intervention:
            jump_timer = intervention.state['jump_timer']
        assert jump_timer > 0



        #tb.save_frame_image(pre_img_path)
        #with AmidarIntervention(tb) as intervention: 
        #    pass
        #tb.save_frame_image(post_img_path, grayscale=False)


        # player_tile
        # regular_mode
        # jump_mode
        # chase_mode
        # enemy_tiles
        # enemy caught
        # any_enemy_caught
        # set box
        # check chase condition
        # chase react
        # set player tile
        # set enemy tile
        # set enemy tiles
        # set mode
        # get enemy protocol 
        # get enemy protocols 
        # set_enemy_protocol
        # get_default_protocol (one for each of 5)
        # get random tile ID
        # get random position 
        # get random direction for tile
        # set enemy protocols 
