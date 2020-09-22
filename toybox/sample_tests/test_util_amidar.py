import random
#import toybox.interventions.amidar as ami
#import toybox.testing.envs.gym as gym
import toybox.testing.models.openai_baselines as oai
import time
import tensorflow as tf

#from scipy.stats import sem
from numpy import ones, mean
from toybox.sample_tests.test_amidar_iclr2020 import AmidarToyboxTestExp, generate_random_dir
from resources.lookup_util import *

class AmidarMonitor(AmidarToyboxTestExp):

    def setUp(self):
        super().setUp()
        self.trials = 1
        self.timeout = 50000

    def resetEnv(self):
        super().resetEnv()

    def onTrialStart(self):
        with ami.AmidarIntervention(self.getToybox()) as intervention:
            intervention.lives = 100

    def onTrialEnd(self):
        pass

    def onTestEnd(self, trials_data):
        pass

    def shouldRecord(self, obj=None):
        return self.tick > 0

    # save state information
    def record(self, obj=None):
        pass

    def runTest(self, model, collection=['n/a']):
        import toybox.testing.envs.gym as gym
        trials_data = []
        for obj in collection:
            with self.subTest(obj=obj):
                for trial in range(self.trials):
                    print('Running trial %d of %s for %s...' % (trial + 1, self.trials, obj))
                    self.onTrialStart()
                    while not self.isDone():
                        # self.goal_tick = self.tick + random.Poisson.()  # sample from Poisson for number of steps to simulate
                        if self.shouldRecord(obj=obj): self.record(obj=obj)
                        # if self.shouldIntervene(obj=obj): self.intervene(obj=obj)
                        #self.env.render()
                        #time.sleep(1 / 30.)
                        self.takeAction(model)
                        self.stepEnv()
                    if self.toReset: self.resetConfig(self.toReset)
                    trials_data.append(self.onTrialEnd())
                    print('Resetting environment.\n')
                    self.resetEnv()
        self.onTestEnd(trials_data)

    def simulate(self):
        print('collecting enemy movement times')
        seed = 42
        path = 'models/AmidarToyboxNoFrameskip-v4.regress.model'
        # # You need to do this if you want to load more than one model with tensorflow
        with tf.compat.v1.Session(graph=tf.Graph()):
            model = oai.getModel(self.env, 'ppo2', seed, path)
            self.runTest(model)


class AmidarCrawler(AmidarMonitor):

    def setUp(self):
        super().setUp()
        self.trials = 1
        self.timeout = 50000
        self.fwd_ct = 0  # how many enemy steps have we seen?
        self.goal_tick = 0  # how many steps are we simulating until?
        self.lookup_table = read_lookup()  # read the default protocol lookup resource
        self.checkpoint_mk = 50

        with ami.AmidarIntervention(self.getToybox()) as intervention:
            tiles = intervention.game.board.tiles
            tw = len(tiles)
            th = len(tiles[0])

            # intialize visitation times to negatives
            self.visitation_record = {}
            for row in tiles:
                for tile in row:
                    if tile.tag != ami.Tile.Empty:
                        tp = intervention.tile_to_tilepoint(tile)
                        tid = tile_to_route_id(intervention,tp.tx, tp.ty)
                        self.visitation_record[tid] = -1
            for enemy in intervention.game.enemies:
                # does quite mark everything
                # should be checking the current tile index
                # and marking 0 for everything from start to the index
                # note: assumes EnemyLookupAI protocol with default tracks
                etp = intervention.worldpoint_to_tilepoint(enemy.position)
                tid = tile_to_route_id(intervention, etp.tx, etp.ty)
                self.visitation_record[tid] = 1

    def resetEnv(self):
        super().resetEnv()
        # set enemy shift using self.fwd_ct
        with ami.AmidarIntervention(self.getToybox()) as intervention:
            shift_vec = [self.fwd_ct % len(self.lookup_table[e.ai.default_route_index])
                         for e in intervention.game.enemies]
            print("resetting with offsets:", shift_vec)
            shift_enemy_defaults(intervention, shift_vec, self.lookup_table)

    def record(self, obj=None):
        # record enemy positions to get ordering of tile visitations
        with ami.AmidarIntervention(self.getToybox()) as intervention:
            for enemy in intervention.game.enemies:
                etp = intervention.worldpoint_to_tilepoint(enemy.position)
                tid = tile_to_route_id(intervention, etp.tx, etp.ty)
                visitation_record = self.visitation_record[tid]
                self.visitation_record[tid] = self.fwd_ct if visitation_record < 0 else visitation_record

    def shouldCheckpoint(self, obj=None): return self.fwd_ct > self.goal_tick

    def checkpoint(self, obj=None):
        # eventually save file with tile visitation info
        n_missing = len(self.visitation_record.values()) - sum([v > -1 for v in self.visitation_record.values()])
        print("tick:", self.fwd_ct, "unmarked:", n_missing)

        # sample next checkpoint
        self.goal_tick = self.fwd_ct + self.checkpoint_mk

    def isDone(self):
        n_missing = len(self.visitation_record.values()) - sum([v > -1 for v in self.visitation_record.values()])
        return n_missing == 0

    def onTestEnd(self, trials_data):
        import os.path as osp
        with open(osp.join('resources','amidar_visitation_times'), 'w') as f:
            with ami.AmidarIntervention(self.getToybox()) as intervention:
                info = ['tile_id', 'tx', 'ty', 'visit_tk']
                f.write(' '.join(info)+'\n')
                for tid in self.visitation_record:
                    tp = tilepoint_lookup(intervention, tid)
                    info = [str(tid), str(tp.tx), str(tp.ty), str(self.visitation_record[tid])]
                    print(' '.join(info))
                    f.write(' '.join(info)+'\n')
                    #print(self.visitation_record)

    def runTest(self, model, collection=['n/a']):
        import toybox.testing.envs.gym as gym
        trials_data = []
        for obj in collection:
            with self.subTest(obj=obj):
                for trial in range(self.trials):
                    print('Running trial %d of %s for %s...' % (trial + 1, self.trials, obj))
                    self.onTrialStart()
                    while not self.isDone():
                        # self.goal_tick = self.tick + random.Poisson.()  # sample from Poisson for number of steps to simulate
                        self.fwd_ct += 1
                        if self.shouldRecord(obj=obj): self.record(obj=obj)
                        if self.shouldCheckpoint(obj=obj): self.checkpoint(obj=obj)
                        #self.env.render()
                        #time.sleep(1 / 30.)
                        self.takeAction(model)
                        self.stepEnv()
                    if self.toReset: self.resetConfig(self.toReset)
                    trials_data.append(self.onTrialEnd())
                    print('Resetting environment.\n')
                    self.resetEnv()
        self.onTestEnd(trials_data)