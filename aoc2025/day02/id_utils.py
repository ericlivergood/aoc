
from typing import Callable


def slice_string(s: str, pieces: int) -> list[str]:
    if len(s) % pieces != 0 or pieces > len(s):
        return []
    size = len(s) // pieces
    n = 0
    slices = []
    while n < len(s):
        slices.append(s[n:n+size])
        n += size
    return slices

def expand_range(range_str: str) -> list[int]:
    start, end = map(int, range_str.split('-'))
    return list(range(start, end + 1))

def is_single_repeat(n: int):
    sliced = slice_string(str(n), 2)
    if(len(sliced) != 2):
        return False
    return sliced[0] == sliced[1]

def contains_repeat(n: int):
    pieces = 2
    max_pieces = len(str(n))
    while pieces <= max_pieces:
        sliced = slice_string(str(n), pieces)
        if(len(sliced) > 0):
            last = None
            had_difference = False
            for s in sliced:
                if s == last or last == None:
                    last = s
                else:
                    had_difference = True
                    break
            
            if not had_difference:
                return True
        
        pieces += 1

    return False


def sum_invalid_ids(ranges: list[str], fn: Callable[[int], bool]):
    sum = 0
    for r in ranges:
        for n in expand_range(r):
            if fn(n):
                sum += n

    return sum