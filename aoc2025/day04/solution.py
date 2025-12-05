"""
Advent of Code 2025 - Day 4
"""
from pathlib import Path
from typing import Any
from day04.paper_grid import from_lines
from utils import read_lines


def part1(data: list[str]) -> int:
    """
    Solve part 1 of the puzzle.

    Args:
        data: Parsed input data

    Returns:
        Solution to part 1
    """
    grid = from_lines(data)
    return grid.count_with_n_adjacent_rolls(4)


def part2(data: list[str]) -> int:
    """
    Solve part 2 of the puzzle.

    Args:
        data: Parsed input data

    Returns:
        Solution to part 2
    """
    grid = from_lines(data)
    count = grid.count_with_n_adjacent_rolls(4, True)
    total = 0
    while count > 0:
        total += count 
        count = grid.count_with_n_adjacent_rolls(4, True)
    
    return total

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
