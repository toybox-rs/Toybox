import unittest

class BehavioralFixture(unittest.TestCase):

  @classmethod
  def setUpClass(cls):
    print("setUpClass")
    cls.setUpEnv()
    cls.setUpModel()

  @classmethod
  def setUpEnv(cls):
    pass

  @classmethod
  def setUpModel(cls):
    pass

  @classmethod
  def tearDownClass(cls):
    print("tearDownClass")
    cls.tearDownEnv()
    cls.tearDownModel()

  @classmethod
  def tearDownEnv(cls):
    pass

  @classmethod
  def tearDownModel(cls):
    pass

  def setUp(self, trials=30, timeout=5e6, record_period=10):
    self.obs = self.env.reset()
    self.trials = trials
    self.timeout = timeout
    self.record_period = record_period
    self.reset_config = None
    self.model = None
  
  def hasTimedOut(self):
    return self.tick > self.timeout

  def log_after_episode(self):
    assert False 

  def log_step(self):
    assert False

  def takeAction(self):
    assert False

  def getToybox(self):
    return self.toybox
