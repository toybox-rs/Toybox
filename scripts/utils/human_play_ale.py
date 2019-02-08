import sys, toybox
from PIL import Image
import argparse
import atari_py
import os
from atari_py.ale_python_interface import ALEInterface
import numpy as np
import argparse
import pygame
import pygame.key
from pygame.locals import *
import pygame.surfarray

def make_action(left, right, up, down, button1, button2):
    fire = button1 or button2
    if down and left and fire:
        return 17
    if down and right and fire:
        return 16
    if up and left and fire:
        return 15
    if up and right and fire:
        return 14
    if down and fire:
        return 13
    if left and fire:
        return 12
    if right and fire:
        return 11
    if up and fire:
        return 10
    if left and down:
        return 9
    if right and down:
        return 8
    if left and up:
        return 7
    if right and up:
        return 6
    if down:
        return 5
    if left:
        return 4
    if right:
        return 3
    if up:
        return 2
    if fire:
        return 1
    return 0


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='human_play for ALE')
    parser.add_argument(
        'game', type=str, help='try one of amidar, breakout, space_invaders')
    parser.add_argument('--scale', type=int, default=2)
    parser.add_argument('--query', type=str, default=None)
    parser.add_argument('--query_args', type=str, default="null")
    parser.add_argument(
        '--seed', help='The random seed for the Atari env (default is 1234).', default=1234, type=int)
    parser.add_argument('--write_actions',
                        type=argparse.FileType('w'), default=sys.stdout)

    args = parser.parse_args()

    ale = ALEInterface()
    __USE_SDL = True
    ale.setInt(b'random_seed', args.seed)
    ale.setFloat(b'repeat_action_probability', 0.0)

    print('Starting up: '+args.game)
    game_path = atari_py.get_game_path(args.game)
    ale.loadROM(str.encode(game_path))
    print('Legal Actions: ', ale.getLegalActionSet())
    pygame.init()

    (w,h) = ale.getScreenDims()
    dim = (w*args.scale,h*args.scale)

    pygame.display.set_mode(dim)
    clock = pygame.time.Clock()
    FPS = 32

    quit = False
    while not quit and not ale.game_over():
        for event in pygame.event.get():
            if event.type == QUIT:
                quit = True
                break
            if event.type == KEYDOWN and event.key == K_ESCAPE:
                quit = True
                break
        key_state = pygame.key.get_pressed()
        
        left = key_state[K_LEFT] or key_state[K_a]
        right = key_state[K_RIGHT] or key_state[K_d]
        up = key_state[K_UP] or key_state[K_w]
        down = key_state[K_DOWN] or key_state[K_s]
        button1 = key_state[K_z] or key_state[K_SPACE]
        button2 = key_state[K_x] or key_state[K_RSHIFT] or key_state[K_LSHIFT]

        act = make_action(left, right, up, down, button1, button2)
        args.write_actions.write('{0}\n'.format(act))
        ale.act(act)    

        image = ale.getScreenRGB2()
        screen = pygame.display.get_surface()
        img = pygame.surfarray.make_surface(np.swapaxes(image,0,1))
        img2x = pygame.transform.scale(img, dim)
        screen.blit(img2x, dest=(0,0))
        pygame.display.update()
        clock.tick(FPS)
    pygame.quit()
    sys.exit()
