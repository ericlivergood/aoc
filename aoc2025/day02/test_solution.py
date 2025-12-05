"""Tests for day 2 solution."""
import pytest
from pathlib import Path
from day02.solution import part1, part2
from utils import read_lines


@pytest.fixture
def example_data():
    """Load example data for tests."""
    day_dir = Path(__file__).parent
    return read_lines(day_dir / "example.txt")


def test_part1_example(example_data):
    """Test part 1 with example data."""
    # TODO: Update expected value once you know it
    expected = 1227775554
    assert part1(example_data) == expected


def test_part2_example(example_data):
    """Test part 2 with example data."""
    # TODO: Update expected value once you know it
    expected = 4174379265
    assert part2(example_data) == expected
