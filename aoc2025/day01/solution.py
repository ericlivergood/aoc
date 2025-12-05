"""
Advent of Code 2025 - Day 1
"""
from pathlib import Path
from typing import List, Tuple, Any

from day01.dial import Dial
from utils import read_lines


def part1(data: list[str]) -> int:
    """
    Solve part 1 of the puzzle.

    Args:
        data: Parsed input data

    Returns:
        Solution to part 1
    """
    dial = Dial()
    for line in data:
        direction, distance = line[0], int(line[1:])
        dial.turn(direction, distance)
    
    return dial.times_stopped_at_zero


def part2(data: Any) -> int:
    """
    Solve part 2 of the puzzle.

    Args:
        data: Parsed input data

    Returns:
        Solution to part 2
    """
    dial = Dial()
    for line in data:
        direction, distance = line[0], int(line[1:])
        dial.turn(direction, distance)
    
    return dial.times_passed_zero


if __name__ == "__main__":
    # Determine the directory where this script is located
    day_dir = Path(__file__).parent

    # Test with example input
    example_data = read_lines(day_dir / "example.txt")
    print(f"Example Part 1: {part1(example_data)}")
    print(f"Example Part 2: {part2(example_data)}")

    print()

    # Solve with real input
    data = read_lines(day_dir / "input.txt")
    print(f"Part 1: {part1(data)}")
    print(f"Part 2: {part2(data)}")
