from gym import logger
import os
import random
import numpy as np
import time

from toybox.sample_tests.test_exp_amidar import EnemyShiftTest

# shift test
def shift_test_suite():
    # add several configurations for nshift, offset_range, noise
    pass

if __name__ == '__main__':
    import unittest

    runner = unittest.TextTestRunner()
    tsuite = unittest.TestSuite()
    #tsuite.addTest(EnemyShiftTest('test_shift_ppo2'))
    tsuite.addTest(EnemyShiftTest('test_shift_all_models'))

    runner.run(tsuite)