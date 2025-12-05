from day03.battery_bank import BatteryBank, find_max, parse_battery_bank_from_input


def test_find_max():
    assert find_max("0", 0) == (0,0)
    assert find_max("1", 0) == (0,1)
    assert find_max("01", 1) == (0,0)
    assert find_max("10", 1) == (0,1)
    assert find_max("101", 1) == (0,1)
    assert find_max("123", 1) == (1,2)
    assert find_max("987654321111111", 1) == (0,9)
    assert find_max("811111111111119", 1) == (0,8)
    assert find_max("234234234234278", 1) == (13,7)
    assert find_max("818181911112111", 1) == (6,9)

def test_find_max_joltage():
    b = parse_battery_bank_from_input("987654321111111") 
    assert b.find_max_joltage(1) == 98
    assert b.find_max_joltage(11) == 987654321111

    b = parse_battery_bank_from_input("811111111111119") 
    assert b.find_max_joltage(1) == 89
    assert b.find_max_joltage(11) == 811111111119

    b = parse_battery_bank_from_input("234234234234278") 
    assert b.find_max_joltage(1) == 78
    assert b.find_max_joltage(11) == 434234234278

    b = parse_battery_bank_from_input("818181911112111") 
    assert b.find_max_joltage(1) == 92
    assert b.find_max_joltage(11) == 888911112111

def test_from_input():
    b = parse_battery_bank_from_input("987654321111111") 
    assert b.joltages == "987654321111111"