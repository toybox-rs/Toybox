import time
import unittest
from abc import ABC, abstractmethod

class BehavioralFixture(unittest.TestCase, ABC):

  @classmethod
  def setUpClass(cls):
    cls.setUpEnv()

  @abstractmethod
  def setUpEnv(cls):
    pass

  @classmethod
  def tearDownClass(cls):
    cls.tearDownEnv()

  @abstractmethod
  def tearDownEnv(cls):
    pass

  def setUp(self, trials=30, timeout=5e6, record_period=10):
    self.obs = self.env.reset()
    self.trials = trials
    self.timeout = timeout
    self.recordInterval = record_period
    self.toReset = None
    self.lives = 10000
    self.tick = 0
    self.done = False
    # Adding this because of problems with gym wrappers 
    # resetting the environment before we have the chance to 
    # grab final state information
    self.final_state = None

  
  def hasTimedOut(self):
    return self.tick > self.timeout

  @abstractmethod
  def onTrialEnd(self):
    assert False 

  @abstractmethod
  def onTestEnd(self):
    assert False

  @abstractmethod
  def shouldIntervene(self, obj=None):
    assert False

  @abstractmethod
  def intervene(self, obj=None):
    assert False

  @abstractmethod
  def takeAction(self, model):
    assert False

  def getToybox(self):
    return self.toybox

  @abstractmethod
  def stepEnv(self):
    assert False

  @abstractmethod
  def resetEnv(self):
    assert False

  def runTest(self, model, collection=['n/a']):
    import toybox.testing.envs.gym as gym
    trials_data = []
    for obj in collection:
      with self.subTest(obj=obj):
        for trial in range(self.trials):
          print('Running trial %d of %s for %s...' % (trial+1, self.trials, obj))
          while not self.isDone():
            if self.shouldIntervene(obj=obj): self.intervene(obj=obj)
            #self.env.render()
            #time.sleep(1/30.)
            self.takeAction(model)
            self.stepEnv()
          if self.toReset: self.resetConfig(self.toReset)
          trials_data.append(self.onTrialEnd()) 
          print('Resetting environment.\n')
          self.resetEnv()
    self.onTestEnd()    
