
from pathlib import Path
from day07.tachyon_beam import TachyonBeam, from_lines
from utils import read_lines


def test_from_lines_finds_start():
    data = ["S...."]
    beam = from_lines(data)
    assert beam.beams[0][0] == 1
    data = ["...S....."]
    beam = from_lines(data)
    assert beam.beams[0][3] == 1
    data = [".......S......."]
    beam = from_lines(data)
    assert beam.beams[0][7] == 1

def test_from_lines_finds_splitters():
    data = ["......"]
    beam = from_lines(data)
    assert len(beam.splitters[0]) == 0
    data = ["^....."]
    beam = from_lines(data)
    assert len(beam.splitters[0]) == 1
    assert beam.splitters[0][0] == 0
    data = ["^....^"]
    beam = from_lines(data)
    assert len(beam.splitters[0]) == 2
    assert beam.splitters[0][0] == 0
    assert beam.splitters[0][1] == 5
    data = ["^^^^^^"]
    beam = from_lines(data)
    assert len(beam.splitters[0]) == 6
    assert beam.splitters[0][0] == 0
    assert beam.splitters[0][1] == 1
    assert beam.splitters[0][2] == 2
    assert beam.splitters[0][3] == 3
    assert beam.splitters[0][4] == 4
    assert beam.splitters[0][5] == 5

def test_from_lines():
    data = [
        "..S..",
        ".....",
        "^^.^^",
        "..^.."
    ]
    beam = from_lines(data)
    assert len(beam.beams) == 1
    assert beam.beams[0][2] == 1

    assert len(beam.splitters) == 4
    assert len(beam.splitters[0]) == 0
    assert len(beam.splitters[1]) == 0
    assert len(beam.splitters[2]) == 4
    assert len(beam.splitters[3]) == 1

def test_progress_no_split():
    beam = TachyonBeam(3, [[],[]], 1)
    assert beam.current_y == 0
    assert beam.beams == [{3: 1}]
    assert beam.unique_split_count == 0
    beam.progress()
    assert beam.current_y == 1
    assert beam.beams == [{3: 1}, {3: 1}]
    assert beam.unique_split_count == 0

def test_progress_split():
    beam = TachyonBeam(3, [[],[3]], 1)
    assert beam.current_y == 0
    assert beam.beams == [{3: 1}]
    assert beam.unique_split_count == 0
    beam.progress()
    assert beam.current_y == 1
    assert beam.beams == [{3: 1},{2: 1, 4: 1}]
    assert beam.unique_split_count == 1

def test_can_progress():
    beam = TachyonBeam(3, [[]], 1)
    assert beam.can_progress() == False

    beam = TachyonBeam(3, [[],[3]], 1)
    assert beam.can_progress() == True
    beam.progress()
    assert beam.can_progress() == False
