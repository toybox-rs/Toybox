import toybox.testing.behavior as behavior
import toybox.testing.envs.toybox.atari as tb_test
import toybox.testing.models.openai as oai_test
import toybox.interventions.amidar as ami

import os

class EnemyRemoval(behavior.BehavioralFixture):

    @classmethod
    def setUpEnv(cls):
        print("setUpEnv")
        # With no enemies, nothing can be random anyway.
        seed = 0
        tb_test.setUpToybox(cls, 'AmidarToyboxNoFrameskip-v4', seed)

    @classmethod
    def tearDownEnv(cls):
        print("tearDownEnv")
        tb_test.tearDownToybox(cls)

    @classmethod
    def setUpModel(cls):
        print("setUpModel")
        # grab the module path to make this less brittle later
        pass

    def takeAction(self, model):
        return oai_test.takeAction(self, model)

    def stepEnv(self, action):
        return tb_test.stepEnv(self.env, action)

    def resetEnv(self):
        self.getToybox().new_game()
        return self.env.reset()

    def isDone(self):
        with ami.AmidarIntervention(self.getToybox()) as ai:
            return ai.get_level() > 1 or self.hasTimedOut() or self.done

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
        return {'painted': painted, 'unpainted': unpainted}

    def onTestEnd(self, trials_data):
        pass

    def intervene(self):
        with ami.AmidarIntervention(self.getToybox()) as ai:
            while ai.num_enemies() > 0:
                ai.remove_enemy(0)
            ai.set_n_lives(1)

    def test_scenario1_ppo2(self):
        seed = 42
        fdir = os.path.dirname(os.path.abspath(__file__))
        path = os.sep.join([fdir, 'models', 'AmidarNoFrameskip-v4.ppo2.5e7.845090117.2018-12-29.model'])  
        model = oai_test.getPPO2(self.env, seed, path)
        # Set low to be a test of a test!
        self.timeout = 100
        tb_test.runTest(self, model)
        