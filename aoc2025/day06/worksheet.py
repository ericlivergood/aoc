from typing import Callable, Generator


class Worksheet:
    def __init__(self, cells: list[list[str]]):
        self.cells = cells
        self.height = len(cells)
        self.width = len(cells[0])

    def generate_column_values(self, x: int) -> Generator[str, None, None]:
        for i in range(0, self.height-1):
            yield int(self.cells[i][x])

    def get_ceph_column_values(self, x: int) -> list[int]:
        width = len(self.cells[0][x])
        result = []

        for i in range(0, width):
            n = ""
            for y in range(0, self.height-1):
                cell = self.cells[y][x]
                c = cell[i]
                if c != " ":
                    n = f"{n}{c}"
            if(len(n) > 0):
                result.append(int(n))
        return result

    def get_column_values(self, x: int) -> list[int]:
        return list(self.generate_column_values(x))

    def get_column_operator(self, x: int) -> str:
        return self.cells[self.height-1][x].strip()
    
    def compute_ceph_column(self, x: int) -> int:
        return self.compute_column(x, self.get_ceph_column_values)

    def compute_column(self, x: int, value_source: Callable[[int], list[int]]=None) -> int:
        if(value_source is None):
            value_source = self.get_column_values

        operator = self.get_column_operator(x)
        result = 1 if operator == "*" else 0
        for v in value_source(x):
            if operator == '*':
                result *= v
            elif operator == '+':
                result += v
        return result

def from_lines(lines: list[str]):
    operators = lines[-1]
    widths = []
    counter = 1
    for c in operators:
        if c not in ['*', '+']:
            counter += 1
        elif counter > 1:
            widths.append(counter)
            counter = 1
    widths.append(counter)
    
    result = []
    for l in lines:
        line = []
        i = 0
        for w in widths:
            line.append(l[i:i+w])
            i += w
        result.append(line)
    w = Worksheet(result)
    return w