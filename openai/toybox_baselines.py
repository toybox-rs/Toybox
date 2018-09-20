import toybox
import toybox.toybox
from baselines.run import main
import sys
import cv2
import numpy as np

if __name__ == '__main__':
    if sys.argv[1] == 'toybox':
        print('Hotpatching call to opencv so it doesn\'t try to convert to greyscale')
        import baselines.common.atari_wrappers as aw
        import baselines.common.vec_env.vec_frame_stack as vfs

        # copied from baselines
        def observation(self, frame):
            # only change is commenting this out -- we are already in grayscale
            #frame = cv2.cvtColor(frame, cv2.COLOR_RGB2GRAY)
            frame = cv2.resize(frame, (self.width, self.height), interpolation=cv2.INTER_AREA)
            return frame[:, :, None]

        def step_wait(self):
            obs, rews, news, infos = self.venv.step_wait()
            self.stackedobs = np.roll(self.stackedobs, shift=-1, axis=-1)
            for (i, new) in enumerate(news):
                if new:
                    self.stackedobs[i] = 0
            #self.stackedobs[..., -obs.shape[-1]:] = obs
            self.stackedobs[...,:] = obs
            return self.stackedobs, rews, news, infos

        aw.WarpFrame.observation = observation
        vfs.VecFrameStack.step_wait = step_wait

    main()

