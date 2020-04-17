from toybox.interventions.amidar import *
import random
#import json
import itertools 

import numpy as np

"""An API for interventions on Amidar."""

mvmt_protocols = ['EnemyLookupAI', 'EnemyPerimeterAI', 'EnemyAmidarMvmt', 'EnemyTargetPlayer', 'EnemyRandomMvmt']
generative_support = ['player_start']
generative_utilities = ['choices', 'weights', 'vars']

class AmidarGenerative(AmidarIntervention):

    def __init__(self, tb, game_name='amidar'):
        # check that the simulation in tb matches the game name.
        AmidarIntervention.__init__(self, tb, game_name)

    def set_partial_config(self, fname): 
        import os

        if os.path.isfile(fname): 
            with open(fname) as f:
                data = json.load(f)
        else: 
            print(fname, "not found; cannot load config")
            raise FileNotFoundError
        self.dirty_config = True
        for k in data.keys(): 
            if k in self.config.keys():
                self.config[k] = data[k]

            elif k == "randomize": 
                self.config[k] = data[k]
                self.set_procedure(data[k])

        # assert for all random elements: choice and weight lists in config are defined 
        for var in self.config['randomize']['vars']:
            assert len(self.config["randomize"]["choices"][var]) > 0

        self.resample_state()                
        print("setting config:", self.config, flush=True)


    def set_procedure(self, data):
        # load randomized variable choices 
        # assign to game generator 
        self.config["randomize"]["choices"] = {}
        self.config["randomize"]["weights"] = {}
        self.config["randomize"]["vars"] = []
        for var in [k for k in data.keys() if k in generative_support]:
            if var == 'player_start':     
                self.config["randomize"]["vars"].append(var)
                var_list, weighted_choice = self.unload_starting_position(data[var])
                # unload choices 
                self.config["randomize"]["choices"][var] = var_list
                # unload weights
                self.config["randomize"]["weights"][var] = weighted_choice if weighted_choice is not None else []
            if False:
            #if var == 'enemy_start': 
                self.config["randomize"]["vars"].append(var)
                var_list, weighted_choice = self.unload_enemy_starting_position(data[var])
                self.config["randomize"]["choices"][var] = var_list
               
        for var in [k for k in data.keys() if not k in generative_support and not k in generative_utilities]:    
            print('Randomizer not supported:', var)


    def tile_wrapper(self, y, x): 
        return {'ty': y, 'tx': x}

    def unload_enemy_starting_position(self, data): 
        load_keys = [k for k in data.keys()] 
        for protocol in load_keys: 
            if protocol == 'reindex': 
                for e in self.num_enemies(): 
                    pass

        return var_list



    def unload_starting_position(self, data): 
        correct_bug_refresh = False
        if "player_start" in data.keys():
            correct_bug_refresh = True 
        if "randomize" in self.config.keys(): 
            if "player_start" in self.config["randomize"].keys() and "default_board_bugs" not in data.keys(): 
                    correct_bug_refresh = True 
        if correct_bug_refresh: 
        	# if default_board_bugs has not been set elsewhere in config, set to False here and reset paint
            self.config["default_board_bugs"] = False
            if 'no_chase' in self.config.keys(): 
                chickens = self.config['no_chase']
            else: 
                chickens = False
            self.empty_board_paint_config(chickens)

        load_keys = [k for k in data.keys()]
        var_list = []
        for load_protocol in load_keys: 
            if load_protocol == 'inf':
                all_tiles, _all_pos = self.collect_all_valid_tiles()
                var_list.extend(all_tiles)
            if load_protocol == 'comb_list':
                var_list.extend(list(itertools.product(data[load_protocol]["xrange"],data[load_protocol]["yrange"])))
            if load_protocol == 'all_except_comb_list': 
                choices = []
                all_tiles, _all_pos = self.collect_all_valid_tiles()
                choices.extend(all_tiles)

                excl_list = []
                excl_list.extend(list(itertools.product(data[load_protocol]["xrange"],data[load_protocol]["yrange"]))) 
                excl_list = set(excl_list)
                var_list.extend([item for item in choices if item not in excl_list])
                print(var_list)

        assert len(var_list) > 0

        # format list to correct dictionary form (input as y, x)
        var_list = [self.tile_wrapper(v[1],v[0]) for v in var_list]

        # filter out inappropriate player positions
        var_list = [v for v in var_list if self.check_is_tile(v)]
        e_pos = self.enemy_tiles()
        var_list = [v for v in var_list if v not in e_pos]

        assert len(var_list) > 0

        return var_list, None


    def resample_state(self, randomize={}):
        SE_config = {}          
        if not bool(randomize): 
            if "randomize" in self.config.keys():
                randomize = self.config["randomize"]
            else: 
                print("No random elements; returning")
                return {}

        for var in randomize['vars']: 
            if var in randomize["weights"].keys() and len(randomize["weights"][var]) > 0: 
                print("weights:", randomize["weights"][var])
                self.config[var] = np.random.choice(randomize["choices"][var], p=randomize["weights"][var])
                print(self.config[var], flush=True)
            else: 
                self.config[var] = np.random.choice(randomize["choices"][var])
                print(self.config[var], flush=True)
            SE_config[var] = self.config[var]
        return SE_config


    def collect_all_valid_tiles(self): 
        # use current config state 
        xmax = len(self.config["board"][0])
        ymax = len(self.config["board"])

        candidates = []
        for x in range(xmax):
            for y in range(ymax):
                candidates.append((x,y))

        valid_pos = []
        valid_tiles = []

        pos = {'tx': -1, 'ty': -1}
        for x_y in candidates: 
            pos['tx'] = x_y[0]
            pos['ty'] = x_y[1]

            if self.check_is_tile(pos): 
                valid_pos.append(pos)
                valid_tiles.append(x_y)

        return valid_tiles, valid_pos




if __name__ == "__main__":
    import argparse 

    parser = argparse.ArgumentParser(description='test Amidar generative fns')
    parser.add_argument('--partial_config', type=str, default="null")
    parser.add_argument('--save_json', type=bool, default=False)

    pre_img_path = "pre_img.jpg"
    post_img_path = "post_img.jpg"

    args = parser.parse_args()

    with Toybox('amidar') as tb:
        state = tb.to_state_json()
        config = tb.config_to_json()

        test_config_fname = "toybox/toybox/generative/amidar_quick_config.json"
        with AmidarGenerative(tb) as rogue: 
            rogue.set_partial_config(test_config_fname)

        if args.save_json:
            # save a sample starting state and config
            with open('toybox/toybox/interventions/defaults/amidar_state_trail.json', 'w') as outfile:
                json.dump(state, outfile)

            with open('toybox/toybox/interventions/defaults/amidar_config_trial.json', 'w') as outfile:
                json.dump(config, outfile)



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