class BatteryBank:
    joltages = ""

    def find_max_joltage(self, n: int) -> int:
        max_str = ""
        pos = 0
        while n >= 0:
            max_pos, max = find_max(self.joltages[pos:], n)
            max_str = f"{max_str}{max}"
            n -= 1
            pos = pos + max_pos+1

        return int(max_str)

def find_max(s: str, buffer_needed: int) -> tuple[int,int]:
    "returns the position of the max value and the max value"
    max = -1
    pos = -1
    i = 0
    for x in s:
        if i > (len(s) - buffer_needed - 1):
            break

        n = int(x)
        if n > max:
            max = n
            pos = i
        i += 1

    return (pos,max)

def parse_battery_bank_from_input(line: str) -> BatteryBank:
    b = BatteryBank()
    b.joltages = line
    return b