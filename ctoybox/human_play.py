import toybox, sys
from toybox import Toybox, Input
from toybox.interventions.base import *

import numpy as np
import argparse
import pygame
import pygame.key
from pygame.locals import *
import pygame.surfarray
import json



if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='human_play for toybox')
    parser.add_argument('game', type=str, help='try one of amidar, breakout, space_invaders')
    parser.add_argument('--scale', type=int, default=2)
    parser.add_argument('--fps', type=int, default=32)
    parser.add_argument('--query', type=str, default=None)
    parser.add_argument('--query_args', type=str, default="null")
    parser.add_argument('--partial_config', type=str, default="null")

    args = parser.parse_args()
    print('Starting up: '+args.game)
    pygame.init()

    with Toybox(args.game) as tb:
        if args.partial_config != "null":    
            with Intervention(tb, args.game) as intervention:
                intervention.set_partial_config(args.partial_config)

        w = tb.get_width()
        h = tb.get_height()

        config_json = tb.config_to_json()
        with open('human_play_config.json', 'w') as fp:
            print(json.dumps(config_json, indent=4, sort_keys=True), file=fp)
        state_json = tb.to_state_json()
        with open('human_play_state.json', 'w') as fp:
            print(json.dumps(state_json, indent=4, sort_keys=True), file=fp)

        dim = (w*args.scale,h*args.scale)

        pygame.display.set_mode(dim)
        clock = pygame.time.Clock()
        FPS = args.fps

        quit = False
        while not quit:
            # close human_play on game over
            if tb.game_over():
                break
            for event in pygame.event.get():
                if event.type == QUIT:
                    quit = True
                    break
                if event.type == KEYDOWN and event.key == K_ESCAPE:
                    quit = True
                    break
            key_state = pygame.key.get_pressed()
            player_input = Input()

            # Explicitly casting to bools because in some versions, the RHS gets converted
            # to ints, causing problems when we load into the associated rust structs.
            player_input.left = bool(key_state[K_LEFT] or key_state[K_a])
            player_input.right = bool(key_state[K_RIGHT] or key_state[K_d])
            player_input.up = bool(key_state[K_UP] or key_state[K_w])
            player_input.down = bool(key_state[K_DOWN] or key_state[K_s])
            player_input.button1 = bool(key_state[K_z] or key_state[K_SPACE])
            player_input.button2 = bool(key_state[K_x] or key_state[K_RSHIFT] or key_state[K_LSHIFT])
                        
            tb.apply_action(player_input)
            if args.query is not None:
                print(args.query, tb.query_state_json(args.query, args.query_args))
            image = tb.get_rgb_frame()
            screen = pygame.display.get_surface()
            img = pygame.surfarray.make_surface(np.swapaxes(image,0,1))
            img2x = pygame.transform.scale(img, dim)
            screen.blit(img2x, dest=(0,0))
            pygame.display.update()
            if key_state[K_TAB]:
                clock.tick(FPS*4)
            else:
                clock.tick(FPS)
    pygame.quit()
    sys.exit()
