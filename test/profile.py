import cProfile
from ctoybox import Toybox
from toybox.interventions.breakout import BreakoutIntervention


def instantiate():
    with Toybox('breakout') as tb:
        with BreakoutIntervention(tb) as intervention:
            pass

cProfile.run('instantiate()')
#  15278049 function calls (15277560 primitive calls) in 7.386 seconds
# removing the call to inspect:
# 10338 function calls (9903 primitive calls) in 0.011 seconds