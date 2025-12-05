"""
Tests for Advent of Code 2025 - Day 4
"""
from pathlib import Path
from day04.solution import part1, part2
from utils import read_lines


def test_part1_example():
    """Test part 1 with example input."""
    day_dir = Path(__file__).parent
    data = read_lines(day_dir / "example.txt")
    # TODO: Update expected value
    assert part1(data) == 13


def test_part2_example():
    """Test part 2 with example input."""
    day_dir = Path(__file__).parent
    data = read_lines(day_dir / "example.txt")
    # TODO: Update expected value
    assert part2(data) == 43
