from toybox.toybox import Toybox, Input
import time
import atari_py as atari
import atari_py.ale_python_interface as ale

N = 100000

for game in ['amidar', 'breakout']:
    # benchmark our games (in grayscale)
    with Toybox(game) as tb:
        scores = []
        startTime = time.time()
        for _ in range(N):
            move_up = Input()
            move_up.up = True
            tb.apply_action(move_up)
            #tb.save_frame_image('%s%03d.png' % (game, i))
            if tb.game_over():
                scores.append(tb.get_score())
                tb.new_game()
        # print('num frames: %d' % len(tb.state))
        endTime = time.time()
        FPS = N / (endTime - startTime)
        print("toybox-%s-FPS: %3.4f" % (game, FPS))
        print("\t", scores[0])

    # benchmark stella
    scores = [0]
    startTime = time.time()
    aleobj = ale.ALEInterface()
    aleobj.loadROM(atari.get_game_path(game))
    aleobj.reset_game()
    score = 0
    for _ in range(N):
        if aleobj.game_over():
            aleobj.reset_game()
            scores.append(score)
            score = 0
        else:
            move_up = 2
            score += aleobj.act(move_up)
    endTime = time.time()
    FPS = N / (endTime - startTime)
    print("atari-%s-FPS: %3.4f" % (game, FPS))
    print("\t", scores[0])
