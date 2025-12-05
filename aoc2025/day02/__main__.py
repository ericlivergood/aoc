"""Entry point for running day02 as a module."""
from pathlib import Path
from day02.solution import part1, part2
from utils import read_lines

# Determine the directory where this package is located
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
