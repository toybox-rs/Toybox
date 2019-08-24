import toybox.testing.behavior as behavior
import toybox.testing.envs.toybox.atari as tb_test
import toybox.testing.models.openai as oai_test
import toybox.interventions.amidar as ami

import os
import time

from scipy.stats import sem
from numpy import mean

class AmidarToyboxTest(behavior.BehavioralFixture):

    @classmethod
    def setUpEnv(cls):
        # With no enemies, nothing can be random anyway.
        seed = 0xdeadbeef
        tb_test.setUpToybox(cls, 'AmidarToyboxNoFrameskip-v4', seed)
    
    @classmethod
    def tearDownEnv(cls):
        tb_test.tearDownToybox(cls)

    def takeAction(self, model):
            return oai_test.takeAction(self, model)

    def stepEnv(self, action):
        return tb_test.stepEnv(self, action)

    def resetEnv(self):
        self.getToybox().new_game()
        return self.env.reset()

    def getMetric(self, info):
        return info[0]['score']

    def isDone(self):
        with ami.AmidarIntervention(self.getToybox()) as ai:
            return ai.get_level() > 1 or self.hasTimedOut() or ai.get_lives() == 1


class EnemyRemovalTest(AmidarToyboxTest):

    def shouldIntervene(self):
        return self.tick == 0

    def onTrialEnd(self, trial_data):
        # An agent trained on ALE should be able to complete at least half of 
        # level 1 before time.
        with ami.AmidarIntervention(self.getToybox()) as ai:
            unpainted = ai.num_tiles_unpainted()
            painted = ai.num_tiles_painted()
        # Set to 0 to ensure it passes.
        self.assertGreaterEqual(painted, 0)
        scores = [t[3] for t in trial_data]
        return {'painted': painted, 'unpainted': unpainted, 
                'score' : {'avg' : mean(scores), 'err': sem(scores)}}

    def onTestEnd(self, trials_data):
        print(trials_data)

    def intervene(self):
        with ami.AmidarIntervention(self.getToybox()) as ai:
            while ai.num_enemies() > 0:
                ai.remove_enemy(0)
            ai.set_n_lives(2)

    def test_scenario1_ppo2(self):
        seed = 42
        fdir = os.path.dirname(os.path.abspath(__file__))
        path = os.sep.join([fdir, 'models', 'AmidarNoFrameskip-v4.ppo2.5e7.845090117.2018-12-29.model'])  
        model = oai_test.getPPO2(self.env, seed, path)
        # Set low to be a test of a test!
        self.timeout = 100
        tb_test.runTest(self, model)
        
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
        tb_test.runTest(self, model)


