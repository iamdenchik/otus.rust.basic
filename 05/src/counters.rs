use crate::types::SignedCounter;
use crate::types::UnsignedCounter;

#[cfg(test)]
mod test_next_signed {
    use super::next_signed;

    #[test]
    fn test() {
        assert_eq!(next_signed(0), 1);
        assert_eq!(next_signed(100), 101);
        assert_eq!(next_signed(9223372036854775806), 9223372036854775807);
        assert_eq!(next_signed(-9223372036854775808), -9223372036854775807);
        assert_eq!(next_signed(isize::MAX - 1).checked_add(2), None);
        assert_eq!(next_signed(isize::MIN + 1).checked_sub(3), None);
    }
}

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

#[cfg(test)]
mod test_next_unsigned {
    use super::next_unsigned;

    #[test]
    fn test() {
        assert_eq!(next_unsigned(0), 1);
        assert_eq!(next_unsigned(100), 101);
        assert_eq!(next_unsigned(18446744073709551614), 18446744073709551615);
        assert_eq!(next_unsigned(usize::MAX - 1).checked_add(1), None);
        assert_eq!(next_unsigned(usize::MIN), 1);
    }
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

#[cfg(test)]
mod test_prev_signed {
    use super::prev_signed;

    #[test]
    fn test() {
        assert_eq!(prev_signed(1), 0);
        assert_eq!(prev_signed(100), 99);
        assert_eq!(prev_signed(9223372036854775807), 9223372036854775806);
        assert_eq!(prev_signed(-9223372036854775807), -9223372036854775808);
        assert_eq!(prev_signed(isize::MAX).checked_add(2), None);
        assert_eq!(prev_signed(isize::MIN + 1).checked_sub(2), None);
    }
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}
