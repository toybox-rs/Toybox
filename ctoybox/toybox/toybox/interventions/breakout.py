from .base import Interventions
"""An API for interventions on breakout."""

class BreakoutInterventions(Interventions):

    def get_paddle_position(self):
        """Given a Toybox object, return the position of the paddle in Breakout."""
        # code to return paddle position here
        return -1