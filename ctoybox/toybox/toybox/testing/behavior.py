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
    self.model = None
  
  def hasTimedOut(self):
    return self.tick > self.timeout

  @abstractmethod
  def onTrialEnd(self):
    assert False 

  @abstractmethod
  def onTestEnd(self):
    assert False

  @abstractmethod
  def takeAction(self, model):
    assert False

  def getToybox(self):
    return self.toybox

  @abstractmethod
  def stepEnv(self):
    assert false

  @abstractmethod
  def resetEnv(self):
    assert false

  def runTest(self, model):
      trials_data = []
      for trial in range(self.trials):
        # for each trial, record the score at mod 10 steps 
        self.tick = 0
        while not self.isDone():
          if self.shouldIntervene():
            self.intervene()
          self.takeAction(model)
          self.stepEnv()
          self.env.render()
          #time.sleep(1/30.0)
        if self.toReset:
          self.resetConfig(self.toReset)
        trials_data.append(self.onTrialEnd())
        self.resetEnv() #  EpisodicLifeEnv problems
      self.onTestEnd(trials_data)    
