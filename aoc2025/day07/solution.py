"""
Advent of Code 2025 - Day 7
"""
from pathlib import Path
from typing import Any
from day07.tachyon_beam import from_lines
from utils import read_lines


def part1(data: list[str]) -> int:
    """
    Solve part 1 of the puzzle.

    Args:
        data: Parsed input data

    Returns:
        Solution to part 1
    """
    beam = from_lines(data)
    while(beam.can_progress()):
        beam.progress()
    return beam.unique_split_count


def part2(data: list[str]) -> int:
    """
    Solve part 2 of the puzzle.

    Args:
        data: Parsed input data

    Returns:
        Solution to part 2
    """
    beam = from_lines(data)
    while(beam.can_progress()):
        beam.progress()
    return beam.get_unique_beam_count()


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
