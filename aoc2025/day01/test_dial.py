"""
Tests for Dial class
"""
import pytest
from .dial import Dial


def test_dial_initialization():
    """Test that dial initializes with correct values."""
    d = Dial()
    assert d.position == 50
    assert d.times_stopped_at_zero == 0


def test_turn_right_simple():
    """Test turning right without wrapping."""
    d = Dial()
    d.turn('R', 10)
    assert d.position == 60
    assert d.times_stopped_at_zero == 0
    assert d.times_passed_zero == 0

def test_turn_left_simple():
    """Test turning left without wrapping."""
    d = Dial()
    d.turn('L', 10)
    assert d.position == 40
    assert d.times_stopped_at_zero == 0
    assert d.times_passed_zero == 0

def test_turn_right_wrap():
    """Test turning right past 99 wraps to 0."""
    d = Dial()
    d.turn('R', 60)  # 50 + 60 = 110 -> wraps to 10
    assert d.position == 10
    assert d.times_stopped_at_zero == 0
    assert d.times_passed_zero == 1

def test_turn_right_wrap_many():
    """Test turning right past 99 wraps to 0."""
    d = Dial()
    d.turn('R', 1000)  # 50 + 60 = 110 -> wraps to 10
    assert d.position == 50
    assert d.times_stopped_at_zero == 0
    assert d.times_passed_zero == 10    

def test_turn_left_wrap():
    """Test turning left past 0 wraps to 99."""
    d = Dial()
    d.turn('L', 60)  # 50 - 60 = -10 -> wraps to 90
    assert d.position == 90
    assert d.times_stopped_at_zero == 0
    assert d.times_passed_zero == 1

def test_turn_left_wrap_many():
    """Test turning right past 99 wraps to 0."""
    d = Dial()
    d.turn('L', 1000)  # 50 + 60 = 110 -> wraps to 10
    assert d.position == 50
    assert d.times_stopped_at_zero == 0
    assert d.times_passed_zero == 10    

def test_stop_at_zero_from_right():
    """Test that stopping at 0 from right turn increments counter."""
    d = Dial()
    d.turn('R', 50)  # 50 + 50 = 100 -> wraps to 0
    assert d.position == 0
    assert d.times_stopped_at_zero == 1
    assert d.times_passed_zero == 1

def test_stop_at_zero_from_left():
    """Test that stopping at 0 from left turn increments counter."""
    d = Dial()
    d.turn('L', 50)  # 50 - 50 = 0
    assert d.position == 0
    assert d.times_stopped_at_zero == 1
    assert d.times_passed_zero == 1

def test_multiple_stops_at_zero():
    """Test that counter increments each time we stop at 0."""
    d = Dial()
    d.turn('R', 50)  # Position 0
    assert d.times_stopped_at_zero == 1
    assert d.times_passed_zero == 1

    d.turn('R', 100)  # Position 0 again (100 % 100 = 0)
    assert d.times_stopped_at_zero == 2
    assert d.times_passed_zero == 2
    
    d.turn('L', 100)  # Position 0 again
    assert d.times_stopped_at_zero == 3
    assert d.times_passed_zero == 3

def test_pass_through_zero_doesnt_count():
    """Test that passing through 0 (not stopping) doesn't increment counter."""
    d = Dial()
    d.turn('R', 60)  # 50 + 60 = 110 -> 10 (passed through 0)
    # This actually stops at 10, not 0, so counter should be 0
    assert d.times_stopped_at_zero == 0
    assert d.times_passed_zero == 1

def test_invalid_direction():
    """Test that invalid direction raises ValueError."""
    d = Dial()
    with pytest.raises(ValueError, match="Invalid direction"):
        d.turn('X', 10)


def test_example_sequence():
    """Test the example sequence from example.txt."""
    d = Dial()
    # L68: 50 - 68 = -18 -> 82
    d.turn('L', 68)
    assert d.position == 82

    # L30: 82 - 30 = 52
    d.turn('L', 30)
    assert d.position == 52
