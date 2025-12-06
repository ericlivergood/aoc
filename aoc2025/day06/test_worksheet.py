from pathlib import Path
from day06.worksheet import Worksheet, from_lines
from utils import read_lines
data = [
    '1  2   3 4   5',
    '6  7   8 9 100',
    '+  *   * + *  '
]
def test_get_column_values():
    w = from_lines(data)
    values = w.get_column_values(0)
    assert values[0] == 1
    assert values[1] == 6

    values = w.get_column_values(1)
    assert values[0] == 2
    assert values[1] == 7

    values = w.get_column_values(2)
    assert values[0] == 3
    assert values[1] == 8

    values = w.get_column_values(3)
    assert values[0] == 4
    assert values[1] == 9

    values = w.get_column_values(4)
    assert values[0] == 5
    assert values[1] == 100

def test_get_column_operator():
    w = from_lines(data)
    assert w.get_column_operator(0) == '+'
    assert w.get_column_operator(1) == '*'
    assert w.get_column_operator(2) == '*'
    assert w.get_column_operator(3) == '+'
    assert w.get_column_operator(4) == '*'

def test_compute_column():
    w = from_lines(data)
    assert w.compute_column(0) == 7
    assert w.compute_column(1) == 14
    assert w.compute_column(2) == 24
    assert w.compute_column(3) == 13
    assert w.compute_column(4) == 500

def test_from_lines():
    w = from_lines(data)
    assert w.height == 3
    assert w.width == 5
    #'1  2   3 4   5',
    assert w.cells[0][0] == "1  "
    assert w.cells[0][1] == "2   "
    assert w.cells[0][2] == "3 "
    assert w.cells[0][3] == "4 "
    assert w.cells[0][4] == "  5"
    
    #'6  7   8 9 100'
    assert w.cells[1][0] == "6  "
    assert w.cells[1][1] == "7   "
    assert w.cells[1][2] == "8 "
    assert w.cells[1][3] == "9 "
    assert w.cells[1][4] == "100"
    
    #'+  *   * + *  '
    assert w.cells[2][0] == "+  "
    assert w.cells[2][1] == "*   "
    assert w.cells[2][2] == "* "
    assert w.cells[2][3] == "+ "
    assert w.cells[2][4] == "*  "

def test_get_ceph_column_values():
    w = from_lines(data)

    values = w.get_ceph_column_values(0)
    assert len(values) == 1
    assert values[0] == 16
    values = w.get_ceph_column_values(1)
    assert len(values) == 1
    assert values[0] == 27
    values = w.get_ceph_column_values(2)
    assert len(values) == 1
    assert values[0] == 38
    values = w.get_ceph_column_values(3)
    assert len(values) == 1
    assert values[0] == 49
    values = w.get_ceph_column_values(4)
    assert len(values) == 3
    assert values[0] == 1
    assert values[1] == 0
    assert values[2] == 50   

def test_get_ceph_column_values_from_example():
    day_dir = Path(__file__).parent
    lines = read_lines(day_dir / "example.txt")    
    w = from_lines(lines)

    values = w.get_ceph_column_values(0)
    assert len(values) == 3
    assert values[0] == 1
    assert values[1] == 24
    assert values[2] == 356       

    values = w.get_ceph_column_values(1)
    assert len(values) == 3
    assert values[0] == 369
    assert values[1] == 248
    assert values[2] == 8       

    values = w.get_ceph_column_values(2)
    assert len(values) == 3
    assert values[0] == 32
    assert values[1] == 581
    assert values[2] == 175       

    values = w.get_ceph_column_values(3)
    assert len(values) == 3
    assert values[0] == 623
    assert values[1] == 431
    assert values[2] == 4           