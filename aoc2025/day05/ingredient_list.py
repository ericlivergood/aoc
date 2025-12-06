class IngredientList:
    def __init__(self, ids: list[int], ranges: list[tuple[int,int]]):
        self.ingredient_ids = ids
        self.fresh_ingredients_ranges = ranges

    def is_fresh(self, id: int) -> bool:
        for r in self.fresh_ingredients_ranges:
            if id >= r[0] and id <= r[1]:
                return True
        return False
    
    def get_fresh_ingredients(self) -> list[int]:
        return list(filter(self.is_fresh, self.ingredient_ids))
    
    def get_nonoverlapping_ranges(self):
        ranges = sorted(self.fresh_ingredients_ranges, key=lambda x: x[0])
        # for r in ranges:
        #     print(r)        
        i = 0
        while i < len(ranges) - 1:
            cur = ranges[i]
            next = ranges[i+1]
            if cur[1] >= next[0]:
                if cur[1] >= next[1]:
                    del ranges[i+1]
                    continue
                else:
                    ranges[i] = (cur[0], next[0]-1)
            i += 1
        # for r in ranges:
        #     print(r)
        return list(filter(lambda x: x is not None, ranges))
    
    def count_all_fresh_ids(self) -> int:
        sum = 0
        for r in self.get_nonoverlapping_ranges():
            sum += (r[1] - r[0]) + 1
        return sum    

def from_lines(lines: list[str]):
    ranges = []
    ids = []
    parsing_ranges = True

    for l in lines:
        if l == "":
            parsing_ranges = False
            continue

        if(parsing_ranges):
            r = l.split("-")
            ranges.append((int(r[0]), int(r[1])))
        else:
            ids.append(int(l))

    return IngredientList(ids, ranges)