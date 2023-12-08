#[cfg(test)]
mod tests {
    use crate::days::day1::{combine_digits, get_first_digit, get_last_digit};

    #[test]
    fn combines_digits() {
        assert_eq!(combine_digits(1, 1), 11);
        assert_eq!(combine_digits(5, 8), 58);
    }

    #[test]
    fn parses_numbers() {
        assert_eq!(get_first_digit("0"), 0);
        assert_eq!(get_first_digit("1"), 1);
        assert_eq!(get_first_digit("2"), 2);
        assert_eq!(get_first_digit("3"), 3);
        assert_eq!(get_first_digit("4"), 4);
        assert_eq!(get_first_digit("5"), 5);
        assert_eq!(get_first_digit("6"), 6);
        assert_eq!(get_first_digit("7"), 7);
        assert_eq!(get_first_digit("8"), 8);
        assert_eq!(get_first_digit("9"), 9);
        assert_eq!(get_first_digit("one"), 1);
        assert_eq!(get_first_digit("two"), 2);
        assert_eq!(get_first_digit("three"), 3);
        assert_eq!(get_first_digit("four"), 4);
        assert_eq!(get_first_digit("five"), 5);
        assert_eq!(get_first_digit("six"), 6);
        assert_eq!(get_first_digit("seven"), 7);
        assert_eq!(get_first_digit("eight"), 8);
        assert_eq!(get_first_digit("nine"), 9);

        assert_eq!(get_last_digit("0"), 0);
        assert_eq!(get_last_digit("1"), 1);
        assert_eq!(get_last_digit("2"), 2);
        assert_eq!(get_last_digit("3"), 3);
        assert_eq!(get_last_digit("4"), 4);
        assert_eq!(get_last_digit("5"), 5);
        assert_eq!(get_last_digit("6"), 6);
        assert_eq!(get_last_digit("7"), 7);
        assert_eq!(get_last_digit("8"), 8);
        assert_eq!(get_last_digit("9"), 9);
        assert_eq!(get_last_digit("one"), 1);
        assert_eq!(get_last_digit("two"), 2);
        assert_eq!(get_last_digit("three"), 3);
        assert_eq!(get_last_digit("four"), 4);
        assert_eq!(get_last_digit("five"), 5);
        assert_eq!(get_last_digit("six"), 6);
        assert_eq!(get_last_digit("seven"), 7);
        assert_eq!(get_last_digit("eight"), 8);
        assert_eq!(get_last_digit("nine"), 9);
    }


}