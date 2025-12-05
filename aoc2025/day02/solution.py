"""
Advent of Code 2025 - Day 2
"""
from pathlib import Path
from typing import Any

from day02.id_utils import contains_repeat, is_single_repeat, sum_invalid_ids
from utils import read_lines


def part1(data: list[str]) -> int:
    """
    Solve part 1 of the puzzle.

    Args:
        data: Parsed input data

    Returns:
        Solution to part 1
    """
    ranges = data[0].split(",")
    return sum_invalid_ids(ranges, is_single_repeat)


def part2(data: list[str]) -> int:
    """
    Solve part 2 of the puzzle.

    Args:
        data: Parsed input data

    Returns:
        Solution to part 2
    """
    ranges = data[0].split(",")
    return sum_invalid_ids(ranges, contains_repeat)


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
