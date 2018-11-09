import gym 
import numpy as np

action_file = "/Users/etosch/Downloads/gameplay/actions.tsv"
M = 10

class ReadFromFileAgent(object):
    def __init__(self, action_space, actions):
        self.action_space = action_space
        self.action_list = actions

    def act(self, observation, reward, done):
      if len(self.action_list):
        next_action = self.action_list[0]
        self.action_list = self.action_list[1:]
        return next_action
      return -1

env = gym.make('BreakoutNoFrameskip-v4')
env.env.ale.setInt(b'random_seed', 9874)

actions = []
with open(action_file, 'r') as f:
  for line in f.readlines():
    action = int(line.split('\t')[-1])
    actions.append(action)



rewards = []
  
for _ in range(M):
  done = False
  reward = 0 
  obs = env.reset()
  agent = ReadFromFileAgent(env.action_space, actions)
  
  while len(agent.action_list):
    action = agent.act(obs, reward, done)
    obs, _reward, done, _ = env.step(action)
    reward += _reward

  rewards.append(reward)

print('Avg/std reward for %d games: %f\n' % (M, np.average(rewards)))

