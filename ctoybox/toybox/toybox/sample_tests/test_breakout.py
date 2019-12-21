import toybox.testing.behavior as behavior
import toybox.testing.envs.toybox.atari as tb_test
import toybox.testing.models.openai as oai_test
import toybox.interventions.breakout as bri



class BreakoutEZChannel(behavior.BehavioralFixture):

    @classmethod
    def setUpEnv(cls):
        print("setUpEnv")
        # With no enemies, nothing can be random anyway.
        seed = 94769380433
        tb_test.setUpToybox(cls, 'BreakoutToyboxNoFrameskip-v4', seed)

    @classmethod
    def tearDownEnv(cls):
        tb_test.tearDownToybox(cls)

    def takeAction(self, model):
        return oai_test.takeAction(self, model)

    def stepEnv(self, action):
        return tb_test.stepEnv(self.env, action)

    def resetEnv(self):
        self.getToybox().new_game()
        return self.env.reset()

    

    # get initial state
    start_state = turtle.toybox.to_json()

    config = start_state['config']
    bricks = start_state['bricks']

    rows = len(start_state['config']['row_scores'])
    cols = len(bricks) // rows

    # Give the ball only one life.
    start_state['lives'] = 1

    # Run for 30 trials

    # First loop over bricks
    for i, brick in enumerate(bricks):
        current_col = i // rows
        col_start_brick = current_col * rows

        # Reset the previous column if we are starting a new one
        if i == col_start_brick and i > 0:
            reset_indices = range(col_start_brick - rows, col_start_brick)
            print(reset_indices)
            for j in reset_indices:
                bricks[j]['alive'] = True
        
        # Set all bricks in the current column to be dead
        for j in range(col_start_brick, col_start_brick + rows):
            bricks[j]['alive'] = False

        # Set our current brick to be alive
        bricks[i]['alive'] = True


        for trial in range(30):
            obs = env.reset()
            # overwrite state inside the env wrappers:
            turtle.toybox.write_json(start_state)
            # Take a step to overwrite anything that's stuck there, e.g., gameover
            obs, _, done, info = env.step(0)

            # keep track of the score as best in case a game_over wipes it out while we're reading
            best_score = 0

            tup = None
            # give it 2 minutes of human time to finish.
            for t in range(7200):
                actions = model.step(obs)[0]
                obs, _, done, info = env.step(actions)
                #env.render()
                #time.sleep(1.0/1920.0)
                score = turtle.toybox.get_score()
                if score > best_score:
                    best_score = score
                fail = turtle.toybox.get_lives() != 1 or turtle.toybox.game_over() 
                hit_brick = not turtle.toybox.rstate.breakout_brick_live_by_index(i)
                tup = (i, current_col, trial, best_score, t, not fail)
                if fail or hit_brick:
                    break

            # how did we do?
            print(tup)
            data.append(tup)
    
    with open('ez_channel_{}.tsv'.format(extra_args['load_path']), 'w') as fp:
        for row in data:
            print('\t'.join([str(r) for r in row]), file=fp)

    env.close()

if __name__ == '__main__':
    main()
