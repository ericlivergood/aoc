
class PaperGrid:
    def __init__(self):
        self.grid = []
        self.width = 0
        self.height = 0

    def get_adjacent_roll_count(self, at_y: int, at_x: int) -> bool:
        count = 0
        for dy in range(-1,2):
            y = at_y + dy
            if(y < 0 or y >= self.height): 
                continue

            for dx in range(-1,2):
                x = at_x + dx          
                if x == at_x and y == at_y:
                    continue
                if(x < 0 or x >= self.width):
                    continue
                count += self.grid[y][x]
        return count               

    def count_with_n_adjacent_rolls(self, n: int, remove: bool = False) -> int:
        found = []
        for y in range(0, self.height):
            for x in range(0, self.width):
                if(self.grid[y][x] == 1):
                    adjacent_count = self.get_adjacent_roll_count(y, x)
                    if adjacent_count < n:
                        found.append((y,x))

        if(remove):
            for f in found:
                self.grid[f[0]][f[1]] = 0
        
        return len(found)

def from_lines(lines: list[str]) -> PaperGrid:
    result = PaperGrid()
    for l in lines:
        line = []
        for c in l:
            if c == '@':
                line.append(1)
            else:
                line.append(0)
        result.grid.append(line)
    result.width = len(result.grid[0])
    result.height = len(result.grid)
    return result
