from day04.paper_grid import from_lines

example = [
    "..@@.@@@@.",
    "@@@.@.@.@@",
    "@@@@@.@.@@",
    "@.@@@@..@.",
    "@@.@@@@.@@",
    ".@@@@@@@.@",
    ".@.@.@.@@@",
    "@.@@@.@@@@",
    ".@@@@@@@@.",
    "@.@.@@@.@."    
]

def test_get_adjacent_roll_count():
    grid = from_lines(example)
    assert grid.get_adjacent_roll_count(0, 0) == 2
    assert grid.get_adjacent_roll_count(0, 1) == 4
    assert grid.get_adjacent_roll_count(0, 2) == 3
    assert grid.get_adjacent_roll_count(0, 3) == 3

    assert grid.get_adjacent_roll_count(3, 0) == 4
    assert grid.get_adjacent_roll_count(3, 1) == 7
    assert grid.get_adjacent_roll_count(3, 2) == 6
    assert grid.get_adjacent_roll_count(3, 3) == 7

def test_from_lines():
    lines = [
        "...",
        ".@.",
        "@@@",
        "..."
    ]

    grid = from_lines(lines)
    assert grid.width == 3
    assert grid.height == 4
    assert grid.grid[0][0] == 0
    assert grid.grid[0][1] == 0
    assert grid.grid[0][2] == 0
    assert grid.grid[1][0] == 0
    assert grid.grid[1][1] == 1
    assert grid.grid[1][2] == 0
    assert grid.grid[2][0] == 1
    assert grid.grid[2][1] == 1
    assert grid.grid[2][2] == 1
