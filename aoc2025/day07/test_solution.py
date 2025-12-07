"""
Tests for Advent of Code 2025 - Day 7
"""
from pathlib import Path
from day07.solution import part1, part2
from utils import read_lines


def test_part1_example():
    """Test part 1 with example input."""
    day_dir = Path(__file__).parent
    data = read_lines(day_dir / "example.txt")
    assert part1(data) == 21


def test_part2_example():
    """Test part 2 with example input."""
    day_dir = Path(__file__).parent
    data = read_lines(day_dir / "example.txt")
    assert part2(data) == 40
