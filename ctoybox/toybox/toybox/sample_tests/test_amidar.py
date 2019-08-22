import toybox.testing.behavior as behavior
import toybox.testing.envs.toybox.atari as tb_test
import toybox.testing.models.openai as oai_test

import os

_seed = 845090117

class EnemyRemoval(behavior.BehavioralFixture):

    @classmethod
    def setUpEnv(cls):
        tb_test.setUpToybox(cls, 'AmidarToyboxNoFrameskip-v4', _seed)

    @classmethod
    def setUpModel(cls):
        # grab the module path to make this less brittle later
        fdir = os.path.dirname(os.path.abspath(__file__))
        path = os.sep.join([fdir, 'models', 'AmidarNoFrameskip-v4.ppo2.5e7.845090117.2018-12-29.model'])  
        oai_test.setUpPPO2(cls, cls.env, _seed, path)

    def takeAction(self):
        return oai_test.takeAction(self)

    def tearDownEnv(cls):
        tb_test.tearDownEnv(cls)

    def test_scenario1(self):
        # replace this stuff with the intervention API later
        config = self.turtle.toybox.config_to_json()
        self.reset_config = config
        config['enemies'] = []
        self.turtle.toybox.write_config_json(config)
        tb_test.runTest(self)
