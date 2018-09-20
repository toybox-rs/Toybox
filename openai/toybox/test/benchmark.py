from toybox.toybox import Toybox, Input
import time
import atari_py as atari
import atari_py.ale_python_interface as ale
import gym

# Copied from https://github.com/openai/gym/blob/master/examples/agents/random_agent.py
class RandomAgent(object):
    """The world's simplest agent!"""
    def __init__(self, action_space):
        self.action_space = action_space

    def act(self, observation, reward, done):
        return self.action_space.sample()

def env_test(game_name, system):
    game = ('Amidar%sNoFrameskip-v0' % ('Toybox' if system=='toybox' else '')) if game_name == 'amidar' \
           else ('Breakout%sNoFrameskip-v0' % ('Toybox' if system=='toybox' else ''))
    #env = gym.envs.atari.AtariEnv(game=game, obs_type='image', frameskip=0)
    print('Gym name:', game)
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
    print('GYM: %s-%s-FPS: %3.4f' % (game_name, system, FPS))
    return FPS


N = 100000

for game in ['amidar', 'breakout']:
    # benchmark our games (in grayscale)
    with Toybox(game) as tb:
        scores = []
        startTime = time.time()
        for _ in range(N):
            move_up = Input()
            move_up.up = True
            tb.apply_action(move_up)
            #tb.save_frame_image('%s%03d.png' % (game, i))
            if tb.game_over():
                scores.append(tb.get_score())
                tb.new_game()
        # print('num frames: %d' % len(tb.state))
        endTime = time.time()
        FPS = N / (endTime - startTime)
        print("toybox-%s-FPS: %3.4f" % (game, FPS))
        #print("\t", scores[0], len(scores))

    # benchmark our env 
    gym_fps = env_test(game, 'toybox')
    slowdown = (FPS - gym_fps) / FPS
    print('Slowdown: %3.4f\n' % slowdown)

    # benchmark stella
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
    print("ale-%s-FPS: %3.4f" % (game, FPS))
    #print("\t", scores[0], len(scores))

    # benchmark openai's env
    gym_fps = env_test(game, 'openai')
    slowdown = (FPS - gym_fps) / FPS
    print('Slowdown: %3.4f\n' % slowdown)

