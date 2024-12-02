#[cfg(test)]
mod tests {
    use crate::days::day7::{Hand, HandType};

    #[test]
    fn compares_hands() {
        let h1 = Hand::new(String::from("32T3K 0"));
        let h2 = Hand::new(String::from("T55J5 0"));
        let h3 = Hand::new(String::from("KK677 0"));
        let h4 = Hand::new(String::from("KTJJT 0"));
        let h5 = Hand::new(String::from("QQQJA 0"));

        assert_eq!(h1.get_hand_type(), HandType::Pair);
        assert_eq!(h2.get_hand_type(), HandType::FourOfAKind);
        assert_eq!(h3.get_hand_type(), HandType::TwoPair);
        assert_eq!(h4.get_hand_type(), HandType::FourOfAKind);
        assert_eq!(h5.get_hand_type(), HandType::FourOfAKind);

        assert_eq!(h4 > h1, true);
        assert_eq!(h4 > h2, true);
        assert_eq!(h4 > h3, true);
        assert_eq!(h4 > h5, true);

        assert_eq!(h5 > h1, true);
        assert_eq!(h5 > h2, true);
        assert_eq!(h5 > h3, true);

        assert_eq!(h2 > h1, true);
        assert_eq!(h2 > h3, true);

        assert_eq!(h3 > h1, true);
    }

    #[test]
    fn five_of_kind() {
        let h1 = Hand::new(String::from("AJAAA 0"));
        let h2 = Hand::new(String::from("AAJAJ 0"));
        let h3 = Hand::new(String::from("JJJAA 0"));
        let h4 = Hand::new(String::from("J9JJJ 0"));

        assert_eq!(h1.get_hand_type(), HandType::FiveOfAKind);
        assert_eq!(h2.get_hand_type(), HandType::FiveOfAKind);
        assert_eq!(h3.get_hand_type(), HandType::FiveOfAKind);
        assert_eq!(h4.get_hand_type(), HandType::FiveOfAKind);
    }

    #[test]
    fn three_of_kind() {
        let h1 = Hand::new(String::from("JJ78Q 0"));
        let h2 = Hand::new(String::from("JJ834 0"));
        assert_eq!(h1.get_hand_type(), HandType::ThreeOfAKind);
        assert_eq!(h2.get_hand_type(), HandType::ThreeOfAKind);
    }

    #[test]
    fn four_of_kind() {
        let h1 = Hand::new(String::from("TJJJQ 0"));
        let h2 = Hand::new(String::from("TTTTQ 0"));
        let h3 = Hand::new(String::from("TTJJQ 0"));
        let h4 = Hand::new(String::from("TTTJQ 0"));
        assert_eq!(h1.get_hand_type(), HandType::FourOfAKind);
        assert_eq!(h2.get_hand_type(), HandType::FourOfAKind);
        assert_eq!(h3.get_hand_type(), HandType::FourOfAKind);
        assert_eq!(h4.get_hand_type(), HandType::FourOfAKind);
    }
}