from toybox import Toybox, Input
import time
import atari_py as atari
import atari_py.ale_python_interface as ale
import gym
import numpy as np
from scipy.stats import sem 

# Copied from https://github.com/openai/gym/blob/master/examples/agents/random_agent.py
class RandomAgent(object):
    """The world's simplest agent!"""
    def __init__(self, action_space):
        self.action_space = action_space

    def act(self, observation, reward, done):
        return self.action_space.sample()    

def get_game_env_name(game_name, system):
    return {
        'toybox' : {
            'amidar' : 'AmidarToyboxNoFrameskip-v4',
            'breakout' : 'BreakoutToyboxNoFrameskip-v4',
            'space_invaders' : 'SpaceInvadersToyboxNoFrameskip-v4'
        },
        'ale' : {
            'amidar' : 'AmidarNoFrameskip-v4',
            'breakout' : 'BreakoutNoFrameskip-v4',
            'space_invaders' : 'SpaceInvadersNoFrameskip-v4'
        }
    }[system][game_name]

def new_fps_dict():
    return {
        'AmidarToyboxNoFrameskip-v4' : {'raw' : [], 'gym': []},
        'BreakoutToyboxNoFrameskip-v4' : {'raw' : [], 'gym': []},
        'SpaceInvadersToyboxNoFrameskip-v4' : {'raw' : [], 'gym': []},
        'AmidarNoFrameskip-v4' : {'raw' : [], 'gym': []},
        'BreakoutNoFrameskip-v4' : {'raw' : [], 'gym': []},
        'SpaceInvadersNoFrameskip-v4': {'raw' : [], 'gym': []}
    }


# FPS estimate -- Toybox raw
def toybox_raw(game, Nsteps, FPS_dict):
    FPS = FPS_dict[get_game_env_name(game, 'toybox')]['raw']
    with Toybox(game) as tb:
        scores = []
        actions = tb.get_legal_action_set()
        startTime = time.time()
        for i in range(Nsteps): 
            move = actions[i % len(actions)]
            if tb.game_over():
                scores.append(tb.get_score())
                tb.new_game()
            else:
                tb.apply_ale_action(move)
        endTime = time.time()
        FPS.append(Nsteps / (endTime - startTime))

def ale_raw(game, Nsteps, FPS_dict):
    FPS = FPS_dict[get_game_env_name(game, 'ale')]['raw']
    scores = []
    aleobj = ale.ALEInterface()
    aleobj.loadROM(atari.get_game_path(game))
    aleobj.reset_game()
    score = 0
    action_set = list(aleobj.getLegalActionSet())
    startTime = time.time()
    for i in range(Nsteps):
        action_index = i % len(action_set)
        action = action_set[action_index]
        if aleobj.game_over():
            aleobj.reset_game()
            scores.append(score)
            score = 0
        else:
            score += aleobj.act(action)
        endTime = time.time()
        FPS.append(Nsteps / (endTime - startTime))


# FPS estimate
def env_test(game, system, Nsteps):
    env_name = get_game_env_name(game, system)
    env = gym.make(env_name)
    agent = RandomAgent(env.action_space)
    obs = env.reset()
    reward, done = 0, False
    startTime = time.time()
    for _ in range(Nsteps):
        action = agent.act(obs, reward, done)
        obs, reward, done, _ = env.step(action)
        if done: 
            obs = env.reset()
    endTime = time.time()
    FPS = Nsteps / (endTime - startTime)
    return FPS

# Random agent estimate
def agent_test(game_name, system, num_games):
    game = get_game_env_name(game_name, system)
    env = gym.make(game)
    agent = RandomAgent(env.action_space)
    obs = env.reset()
    scores, done = [], None
    for _ in range(num_games):
        obs = env.reset()
        reward = 0
        score = 0
        while not done:
            action = agent.act(obs, reward, done)
            obs, _reward, done, info = env.step(action)
            # Without OpenAI wrappers, reward is change in score
            score += _reward
        scores.append(score)
    print('Avg/err scores for %d games of %s: %f\n\t%f\n\n' % (num_games, game, np.average(scores), sem(scores)))
                

def main(Nsteps, num_games):
    FPS_dict = new_fps_dict()
    for game in ['amidar', 'breakout', 'space_invaders']:
        tb_env_name = get_game_env_name(game, 'toybox')
        ale_env_name = get_game_env_name(game, 'ale')

        # benchmark raw fps
        for _ in range(30):
            toybox_raw(game, Nsteps, FPS_dict)
            ale_raw(game, Nsteps, FPS_dict)

        # benchmark envs 
        FPS_tb = FPS_dict[tb_env_name]['gym']
        FPS_ale = FPS_dict[ale_env_name]['gym']
        for _ in range(30):
            tb_gym_fps = env_test(game, 'toybox', Nsteps)
            FPS_tb.append(tb_gym_fps)
            ale_gym_fps = env_test(game, 'ale', Nsteps)
            FPS_ale.append(ale_gym_fps)

        tb_fps = FPS_dict[tb_env_name]['raw']
        ale_fps = FPS_dict[ale_env_name]['raw']

        tb_avg = np.average(tb_fps)
        ale_avg = np.average(ale_fps)

        tb_sterr = sem(tb_fps)
        ale_sterr = sem(ale_fps)
        print("toybox-%s-FPS:\n\t %3.4f\n\t%3.4f" % (game, tb_avg, tb_sterr))
        print("ale-%s-FPS:\n\t %3.4f\n\t %3.4f" % (game, ale_avg, ale_sterr))

        tb_before = np.average(FPS_dict[tb_env_name]['raw'])
        tb_after = np.average(FPS_tb)
        print('GYM: %s-toybox-FPS:\n\t %3.4f\n\t %3.4f' % (game, tb_after, sem(FPS_tb)))
        slowdown = (tb_before - tb_after) / tb_before
        print('Slowdown: %3.4f\n' % slowdown)

        ale_before = np.average(FPS_dict[ale_env_name]['raw'])
        ale_after = np.average(FPS_ale)
        print('GYM: %s-ALE-FPS:\n\t %3.4f\n\t %3.4f' % (game, ale_after, sem(FPS_ale)))
        slowdown = (ale_before - ale_after) / ale_before
        print('Slowdown: %3.4f\n' % slowdown)

        for system in ['toybox', 'ale']:
            agent_test(game, system, 30)

if __name__ == '__main__':
    main(10000, 30)