import toybox
from toybox.interventions.base import *
from toybox.generative.amidar import * 

def collect_all_valid_tiles(partial_config=None): 
    with Toybox('amidar') as tb:
        with AmidarGenerative(tb) as intervention:
            if partial_config:    
                intervention.set_partial_config(partial_config)
            xmax = len(intervention.config["board"][0])
            ymax = len(intervention.config["board"])

        candidates = []
        for x in range(xmax):
            for y in range(ymax):
                candidates.append((x,y))

        valid_tiles = []
        pos = {'tx': -1, 'ty': -1}
        with AmidarIntervention(tb) as intervention: 
            for x_y in candidates: 
                pos['tx'] = x_y[0]
                pos['ty'] = x_y[1]

                if intervention.check_is_tile(pos): 
                    valid_tiles.append(x_y)

    return valid_tiles



if __name__ == "__main__":
    import argparse     

    parser = argparse.ArgumentParser(description='test Amidar interventions')
    parser.add_argument('--partial_config', type=str, default=None)

    args = parser.parse_args()

    f = open("tileset", "w")
    valid_tiles = collect_all_valid_tiles(args.partial_config)
    for pos in valid_tiles: 
        f.write(str(pos[0])+","+str(pos[1]) + " ")
    f.close()
