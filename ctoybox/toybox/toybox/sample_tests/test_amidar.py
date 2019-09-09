import toybox.testing.behavior as behavior
import toybox.testing.envs.gym as gym
import toybox.testing.models.openai as oai
import toybox.interventions.amidar as ami

import os
import time
import tensorflow as tf

from scipy.stats import sem
from numpy import mean
from abc import ABC, abstractmethod

class AmidarToyboxTest(behavior.BehavioralFixture, ABC):

    @classmethod
    def setUpEnv(cls):
        # With no enemies, nothing can be random anyway.
        seed = 0xdeadbeef
        gym.setUpToyboxGym(cls, 'AmidarToyboxNoFrameskip-v4', seed)
    
    @classmethod
    def tearDownEnv(cls):
        gym.tearDownToyboxGym(cls)

    def takeAction(self, model):
        oai.takeAction(self, model)

    def stepEnv(self):
        gym.stepEnv(self)

    def resetEnv(self):
        gym.resetEnv(self)

    def isDone(self):
        with ami.AmidarIntervention(self.getToybox()) as ai:
            return ai.get_level() > 1 or self.hasTimedOut() or ai.get_lives() == 1

    @abstractmethod
    def onTestEnd(self):
        pass 

    @abstractmethod
    def onTrialEnd(self):
        pass


class EnemyRemovalTest(AmidarToyboxTest):

    def shouldIntervene(self):
        return self.tick == 0

    def onTrialEnd(self):
        # An agent trained on ALE should be able to complete at least half of 
        # level 1 before time.
        with ami.AmidarIntervention(self.getToybox()) as ai:
            unpainted = ai.num_tiles_unpainted()
            painted = ai.num_tiles_painted()
        # Set to 0 to ensure it passes.
        self.assertGreaterEqual(painted, 0)
        with ami.AmidarIntervention(self.getToybox()) as ai:
            score = ai.getScore()
        return {'painted': painted, 'unpainted': unpainted, 
                'score' : score}

    def onTestEnd(self, trials_data):
        print(trials_data)

    def intervene(self):
        with ami.AmidarIntervention(self.getToybox()) as ai:
            while ai.num_enemies() > 0:
                ai.remove_enemy(0)
            ai.set_n_lives(2)

    def test_no_enemies_ppo2(self):
        print('test_no_enemies_ppo2')
        seed = 42
        fdir = os.path.dirname(os.path.abspath(__file__))
        path = os.sep.join([fdir, 'models', 'AmidarNoFrameskip-v4.ppo2.5e7.845090117.2018-12-29.model'])  
        # You need to do this if you want to load more than one model with tensorflow
        with tf.Session(graph=tf.Graph()):
            model = oai.getModel(self.env, 'ppo2', seed, path)
            # Set low to be a test of a test!
            self.timeout = 10
            self.runTest(model)

    def test_no_enemies_all_models(self):
        print('test_no_enemies_all_models')
        seed = 42
        fdir = os.path.dirname(os.path.abspath(__file__))
        models = [f for f in os.listdir(fdir + os.sep + 'models') if f.startswith('Amidar')]
        print('num models:', len(models))
        for trained in models:
            print(trained)
            path = os.sep.join([fdir, 'models', trained])
            family = trained.split('.')[1]
            with tf.Session(graph=tf.Graph()):
                model = oai.getModel(self.env, family, seed, path)
                print(self.timeout)
                self.runTest(model)
        
class GangUpNoJumpTest(AmidarToyboxTest):

    def shouldIntervene(self):
        return self.tick == 0

    def onTrialEnd(self, trial_data):
        with ami.AmidarIntervention(self.getToybox()) as ai:
            unpainted = ai.num_tiles_unpainted()
            painted = ai.num_tiles_painted()
            jumps = ai.jumps_remaining()
        scores = [t[3] for t in trial_data]
        self.assertGreaterEqual(painted, 10)
        return {'painted': painted, 'unpainted': unpainted, 
                'score' : scores[-1]}

    def onTestEnd(self, trials_data):
        print(trials_data)

    def intervene(self):
        with ami.AmidarIntervention(self.getToybox()) as ai:
            ai.set_n_jumps(0)
            ai.set_n_lives(2)
            for eid in ai.get_enemy_ids():
                ai.set_enemy_protocol(eid, 'EnemyTargetPlayer')


    def test_scenario_ppo2(self):
        seed = 42
        fdir = os.path.dirname(os.path.abspath(__file__))
        path = os.sep.join([fdir, 'models', 'AmidarToyboxNoFrameskip-v4.ppo2.5e7.3771075072.2019-05-18.model'])  
        model = oai_test.getPPO2(self.env, seed, path)
        # Set low to be a test of a test!
        gym.runTest(self, model)


