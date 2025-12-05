"""Common utility functions for Advent of Code solutions."""


def read_lines(filename: str) -> list[str]:
    """
    Read a file and return its lines as a list of strings.

    Each line will have trailing whitespace (including newline) stripped.
    Empty lines are preserved.

    Args:
        filename: Path to the input file

    Returns:
        List of strings, one per line in the file

    Example:
        >>> lines = read_lines("input.txt")
        >>> print(lines[0])  # First line of the file
    """
    with open(filename, 'r') as f:
        return [line.rstrip('\n') for line in f]
