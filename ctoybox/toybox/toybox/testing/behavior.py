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
    self.model = None
    self.lives = 10000
  
  def hasTimedOut(self):
    return self.tick > self.timeout

  @abstractmethod
  def onTrialEnd(self):
    assert False 

  @abstractmethod
  def onTestEnd(self):
    assert False

  @abstractmethod
  def intervene(self):
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
      trials_data = []
      if collection:
        for obj in collection:
          with self.subTest(obj=obj):
            for trial in range(self.trials):
              # for each trial, record the score at mod 10 steps 
              print('Running trial %d...' % trial)
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
              # commenting out the reset env because I am seeing a lot of re-initialization of the game state
              print('Resetting environment.')
              self.resetEnv() #  EpisodicLifeEnv problems
      self.onTestEnd()    
