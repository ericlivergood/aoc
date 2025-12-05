"""
Dial object for Day 1 puzzle
"""


import math


class Dial:
    """
    A dial with positions 0-99 that can be turned left or right.
    Tracks the current position and counts how many times it stops at 0.
    """

    def __init__(self):
        """Initialize the dial at position 50 with 0 stops at position 0."""
        self.position = 50
        self.times_stopped_at_zero = 0
        self.times_passed_zero = 0

    def turn(self, direction: str, amount: int) -> None:
        """
        Turn the dial in the specified direction by the given amount.

        The dial wraps around at 0 and 100 (positions are 0-99).
        If the final position is 0, increment the times_stopped_at_zero counter.
        Each time the dial wraps past 0, increment times_passed_zero.

        Args:
            direction: 'L' for left (subtract) or 'R' for right (add)
            amount: Number of positions to turn
        """
        if(amount >= 100):
            self.times_passed_zero += 1
            self.turn(direction, amount - 100)
            return
        
        start = self.position
        if direction == 'R':
            # Turn right (add)
            self.position = self.position + amount
            if self.position >= 100:
                n = math.ceil(amount / 100)
                print(f"{direction}{amount} past 0 {n} times")
                self.times_passed_zero += n            
        elif direction == 'L':
            # Turn left (subtract)
            self.position = self.position - amount
            if(self.position <= 0 and start != 0):
                n = math.ceil(amount / 100)
                print(f"{direction}{amount} past 0 {n} times")
                self.times_passed_zero += n            
        else:
            raise ValueError(f"Invalid direction: {direction}. Must be 'L' or 'R'.")

        self.position = self.position % 100

        # Check if we stopped at 0
        if self.position == 0:
            self.times_stopped_at_zero += 1

    def __repr__(self) -> str:
        """String representation of the dial state."""
        return f"Dial(position={self.position}, times_stopped_at_zero={self.times_stopped_at_zero})"
