from toybox.toybox import Toybox, Input
import time
import atari_py as atari
import atari_py.ale_python_interface as ale
import gym
import numpy as np
from scipy.stats import sem 

N = 100000
M = 100

# Copied from https://github.com/openai/gym/blob/master/examples/agents/random_agent.py
class RandomAgent(object):
    """The world's simplest agent!"""
    def __init__(self, action_space):
        self.action_space = action_space

    def act(self, observation, reward, done):
        return self.action_space.sample()

class AlwaysMoveLeftAgent(object):
    """The world's simplest agent!"""
    def __init__(self, action_space):
        self.action_space = action_space

    def act(self, observation, reward, done):
        return self.action_space[0]
    

def get_game(game_name, system):
    return ('Amidar%sNoFrameskip-v0' % ('Toybox' if system=='toybox' else '')) if game_name == 'amidar' \
           else ('Breakout%sNoFrameskip-v0' % ('Toybox' if system=='toybox' else ''))

# FPS estimate
def env_test(game_name, system):
    game = get_game(game_name, system)
    #env = gym.envs.atari.AtariEnv(game=game, obs_type='image', frameskip=0)
    env = gym.make(game)
    agent = RandomAgent(env.action_space)
    obs = env.reset()
    reward = 0 
    done = None
    startTime = time.time()
    for _ in range(N):
        action = agent.act(obs, reward, done)
        obs, reward, done, _ = env.step(action)
        if done: 
            obs = env.reset()
    endTime = time.time()
    FPS = N / (endTime - startTime)
    return FPS

# Random agent estimate
def agent_test(game_name, system):
    game = get_game(game_name, system)
    print('Gym name:', game)
    env = gym.make(game)
    agent = RandomAgent(env.action_space)
    obs = env.reset()
    rewards = []
    done = None
    for _ in range(M):
        obs = env.reset()
        reward = 0
        while not done:
            action = agent.act(obs, reward, done)
            obs, _reward, done, _ = env.step(action)
            reward += _reward
        rewards.append(reward)
    print('Avg/std reward for %d games of %s: %f\n\t%f\n\n' % (M, game, np.average(rewards), sem(rewards)))
                

FPS_dict = {
    'toybox-amidar': [],
    'toybox-breakout': [],
    'gym-toybox-amidar': [],
    'gym-toybox-breakout': [],
    'ale-amidar': [],
    'ale-breakout': [],
    'gym-ale-amidar': [],
    'gym-ale-breakout': []
}


for game in ['amidar', 'breakout']:
    # benchmark our games 
    for _ in range(30):
        with Toybox(game) as tb:
            scores = []
            startTime = time.time()
            for i in range(N):
                move = Input()
                dir = i % 5
                but = i % 3
                if dir == 0:
                    move.up = True
                elif dir == 1:
                    move.down = True
                elif dir == 2:
                    move.left = True
                elif dir == 3:
                    move.down = True
                if but == 0:
                    move.button1 = True
                elif but == 1:
                    move.button2 = True

                tb.apply_action(move)
                #tb.save_frame_image('%s%03d.png' % (game, i))
                if tb.game_over():
                    scores.append(tb.get_score())
                    tb.new_game()
            # print('num frames: %d' % len(tb.state))
            endTime = time.time()
            FPS = N / (endTime - startTime)
            #print("\t", scores[0], len(scores))
            FPS_dict['toybox-'+game].append(FPS)
    FPS = FPS_dict['toybox-'+game]
    avg = np.average(FPS)
    sterr = sem(FPS)
    print("toybox-%s-FPS:\n\t %3.4f\n\t%3.4f" % (game, avg, sterr))

    # benchmark our env 
    for _ in range(30):
        gym_fps = env_test(game, 'toybox')
        FPS_dict['gym-toybox-'+game].append(gym_fps)
    before = avg
    FPS = FPS_dict['gym-toybox-'+game]
    avg = np.average(FPS)
    print('GYM: %s-%s-FPS:\n\t %3.4f\n\t %3.4f' % (game, system, avg, sem(FPS)))
    slowdown = (before - avg) / before
    print('Slowdown: %3.4f\n' % slowdown)

    # benchmark stella
    for _ in range(30):
        scores = [0]
        startTime = time.time()
        aleobj = ale.ALEInterface()
        aleobj.loadROM(atari.get_game_path(game))
        aleobj.reset_game()
        score = 0
        action_set = list(aleobj.getLegalActionSet())
        for i in range(N):
            action_index = i % len(action_set)
            action = action_set[action_index]
            if aleobj.game_over():
                aleobj.reset_game()
                scores.append(score)
                score = 0
            else:
                score += aleobj.act(action)
        endTime = time.time()
        FPS = N / (endTime - startTime)
        FPS_dict['ale-'+game].append(FPS)
    FPS = FPS_dict['ale-'+game]
    avg = np.average(FPS)
    sterr = sem(FPS)
    print("ale-%s-FPS:\n\t %3.4f\n\t %3.4f" % (game, avg, sterr))
    #print("\t", scores[0], len(scores))

    # benchmark openai's env
    before = FPS
    for _ in range(30):
        gym_fps = env_test(game, 'openai')
        FPS_dict['gym-ale-'+game].append(gym_fps)
    FPS = FPS_dict['gym-ale-'+game]
    avg = np.average(FPS)
    print('GYM: %s-%s-FPS:\n\t %3.4f\n\t %3.4f' % (game, system, avg, sem(FPS)))
    slowdown = (before - avg) / before
    print('Slowdown: %3.4f\n' % slowdown)

    for system in ['toybox', 'ale']:
        agent_test(game, system)