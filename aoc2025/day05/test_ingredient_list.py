
from day05.ingredient_list import IngredientList, from_lines

lines = [
    "3-5",
    "10-14",
    "16-20",
    "12-18",
    "",
    "1",
    "5",
    "8",
    "11",
    "17",
    "32"        
]


def test_is_fresh():
    i = from_lines(lines)    
    assert i.is_fresh(1) == False
    assert i.is_fresh(5) == True
    assert i.is_fresh(8) == False
    assert i.is_fresh(11) == True
    assert i.is_fresh(17) == True
    assert i.is_fresh(32) == False

def test_get_fresh_ingredients():
    i = from_lines(lines)  
    fresh = i.get_fresh_ingredients()
    assert len(fresh) == 3
    assert fresh[0] == 5
    assert fresh[1] == 11
    assert fresh[2] == 17
    
def test_from_lines():
    i = from_lines(lines)
    assert len(i.ingredient_ids) == 6
    assert i.ingredient_ids[0] == 1
    assert i.ingredient_ids[1] == 5
    assert i.ingredient_ids[2] == 8
    assert i.ingredient_ids[3] == 11
    assert i.ingredient_ids[4] == 17
    assert i.ingredient_ids[5] == 32

    assert len(i.fresh_ingredients_ranges) == 4
    assert i.fresh_ingredients_ranges[0] == (3, 5)
    assert i.fresh_ingredients_ranges[1] == (10, 14)
    assert i.fresh_ingredients_ranges[2] == (16, 20)
    assert i.fresh_ingredients_ranges[3] == (12, 18)


def test_get_nonoverlapping_ranges():
    i = from_lines(lines)
    r = i.get_nonoverlapping_ranges()
    assert len(r) == 4
    assert r[0] == (3,5)
    assert r[1] == (10,11)  
    assert r[2] == (12,15)  
    assert r[3] == (16,20)

    ranges = [
        (1, 10),
        (5, 8),
        (9, 12)
    ]

    i = IngredientList([], ranges)
    r = i.get_nonoverlapping_ranges()
    assert len(r) == 2
    assert r[0] == (1,8)
    assert r[1] == (9,12)