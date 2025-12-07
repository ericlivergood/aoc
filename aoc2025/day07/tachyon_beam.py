class TachyonBeam:
    def __init__(self, start_x: int, splitters: list[list[int]], width: int):
        self.beams = [{start_x: 1}]
        self.current_y = 0
        self.splitters = splitters
        self.splitters_hit_count = [0]
        self.unique_split_count = 0
        self.width = width

    def get_unique_beam_count(self) -> int:
        beams = self.beams[-1]
        return sum(beams.values())

    def can_progress(self) -> bool:
        return self.current_y < len(self.splitters)-1
    
    def progress(self):
        next_beams = {}
        splitters = self.splitters[self.current_y+1]
        for b in self.beams[self.current_y].keys():
            count = self.beams[self.current_y][b]
            if b in splitters:
                self.unique_split_count += 1
                next_beams[b-1] = next_beams.get(b-1, 0) + count
                next_beams[b+1] = next_beams.get(b+1, 0) + count
            else:
                next_beams[b] = next_beams.get(b, 0) + count

        self.beams.append(next_beams)
        self.current_y += 1

    def print(self):
        for y in range(0, len(self.splitters)):
            line = ""
            for x in range(0, self.width):
                if y < len(self.beams) and x in self.beams[y]:
                    line += "|"
                elif x in self.splitters[y]:
                    line += "^"
                else:
                    line += "."
            print(line)
            print()
            


def from_lines(data: list[str]):
    start = 0
    splitters = []
    width = len(data[0])
    for y in range(0, len(data)):
        line_splitters = []
        for x in range(0, len(data[y])):
            c = data[y][x]
            if c == "S":
                start = x
            elif c == "^":
                line_splitters.append(x)

        splitters.append(line_splitters)

    return TachyonBeam(start, splitters, width)
