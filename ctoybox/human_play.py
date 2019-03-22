import toybox, sys
from toybox.toybox import Toybox, Input
import numpy as np
import argparse
import pygame
import pygame.key
from pygame.locals import *
import pygame.surfarray
import json

AMIDAR_TILE_TO_WORLD = 64

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='human_play for toybox')
    parser.add_argument('game', type=str, help='try one of amidar, breakout, space_invaders')
    parser.add_argument('--scale', type=int, default=2)
    parser.add_argument('--query', type=str, default=None)
    parser.add_argument('--query_args', type=str, default="null")

    args = parser.parse_args()
    print('Starting up: '+args.game)
    pygame.init()

    with Toybox(args.game) as tb:
        w = tb.get_width()
        h = tb.get_height()

        config_json = tb.config_to_json()
        with open('config.json', 'w') as cf:
            print(json.dumps(config_json, indent=3), file=cf)
        state_json = tb.to_state_json()
        with open('start-state.json', 'w') as sf:
            print(json.dumps(state_json, indent=3), file=sf)
        
        board_tiles = state_json['board']['tiles']
        print(len(board_tiles))
        print(len(board_tiles[0]))

        state_json['board']['tiles'] = board_tiles[:14]
        state_json['player']['position']['y'] //= 2;
        state_json['player']['history'] = []
        state_json['player_start']['ty'] //= 2;

        config_json['start_lives'] = 0
        state_json['lives'] = 0

        config_json['render_images'] = False
        tb.write_config_json(config_json);
        tb.write_state_json(state_json);


        dim = (w*args.scale,h*args.scale)

        pygame.display.set_mode(dim)
        clock = pygame.time.Clock()
        FPS = 32

        quit = False
        while not quit:
            for event in pygame.event.get():
                if event.type == QUIT:
                    quit = True
                    break
                if event.type == KEYDOWN and event.key == K_ESCAPE:
                    quit = True
                    break
            key_state = pygame.key.get_pressed()
            player_input = Input()
            player_input.left = key_state[K_LEFT] or key_state[K_a]
            player_input.right = key_state[K_RIGHT] or key_state[K_d]
            player_input.up = key_state[K_UP] or key_state[K_w]
            player_input.down = key_state[K_DOWN] or key_state[K_s]
            player_input.button1 = key_state[K_z] or key_state[K_SPACE]
            player_input.button2 = key_state[K_x] or key_state[K_RSHIFT] or key_state[K_LSHIFT]

            tb.apply_action(player_input)


            state_json = tb.to_state_json()
            print(state_json['player']['position'])
            sys.stdout.flush();

            if args.query is not None:
                print(args.query, tb.query_state_json(args.query, args.query_args))
            image = tb.get_rgb_frame()
            screen = pygame.display.get_surface()
            img = pygame.surfarray.make_surface(np.swapaxes(image,0,1))
            img2x = pygame.transform.scale(img, dim)
            screen.blit(img2x, dest=(0,0))
            pygame.display.update()
            clock.tick(FPS)
    pygame.quit()
    sys.exit()
