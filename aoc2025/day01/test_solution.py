"""
Tests for Day 1 solution
"""
import pytest
from pathlib import Path

from utils import read_lines
from .solution import part1, part2


def test_part1_example():
    """Test part 1 with the example input."""
    day_dir = Path(__file__).parent
    data = read_lines(day_dir / "example.txt")
    result = part1(data)

    # TODO: Update expected value based on puzzle example
    expected = 3
    assert result == expected, f"Expected {expected}, got {result}"


def test_part2_example():
    """Test part 2 with the example input."""
    day_dir = Path(__file__).parent
    data = read_lines(day_dir / "example.txt")
    result = part2(data)

    # TODO: Update expected value based on puzzle example
    expected = 6
    assert result == expected, f"Expected {expected}, got {result}"


# Optional: Add specific unit tests for helper functions
# def test_parse_input():
#     """Test input parsing logic."""
#     pass
