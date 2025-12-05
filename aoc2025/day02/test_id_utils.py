from day02.id_utils import contains_repeat, expand_range, is_single_repeat, slice_string, sum_invalid_ids

def test_expand_range():
    range = expand_range("11-22")
    assert len(range) == 12
    assert range[0] == 11
    assert range[11] == 22

    range = expand_range("100-100")
    assert len(range) == 1
    assert range[0] == 100

def test_is_invalid_id():
    assert is_single_repeat(12345) == False
    assert is_single_repeat(11) == True
    assert is_single_repeat(22) == True
    assert is_single_repeat(1188511885) == True
    assert is_single_repeat(222222) == True
    assert is_single_repeat(446446) == True
    assert is_single_repeat(38593859) == True

def test_contains_repeat():
    assert contains_repeat(11) == True
    assert contains_repeat(22) == True
    assert contains_repeat(1188511885) == True
    assert contains_repeat(222222) == True
    assert contains_repeat(446446) == True
    assert contains_repeat(38593859) == True
    assert contains_repeat(111) == True
    assert contains_repeat(1111111) == True
    assert contains_repeat(12345) == False

def test_sum_invalid_ids():
    ranges = ["11-22"]
    sum = sum_invalid_ids(ranges, is_single_repeat)
    assert sum == 33

def test_slice_string():
    slices = slice_string("123", 2)
    assert len(slices) == 0

    slices = slice_string("1234", 2)
    assert len(slices) == 2
    assert slices[0] == "12"
    assert slices[1] == "34"


    slices = slice_string("123456789", 3)
    assert len(slices) == 3
    assert slices[0] == "123"
    assert slices[1] == "456"
    assert slices[2] == "789"


    slices = slice_string("1234567890", 2)
    assert len(slices) == 2
    assert slices[0] == "12345"
    assert slices[1] == "67890"


    slices = slice_string("1234567890", 5)
    assert len(slices) == 5
    assert slices[0] == "12"
    assert slices[1] == "34"
    assert slices[2] == "56"
    assert slices[3] == "78"
    assert slices[4] == "90"


    slices = slice_string("123", 3)
    assert len(slices) == 3
    assert slices[0] == "1"
    assert slices[1] == "2"
    assert slices[2] == "3"