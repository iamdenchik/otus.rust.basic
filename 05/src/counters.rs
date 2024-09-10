pub type SignedCounter = isize;
pub type UnsignedCounter = usize;

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

pub fn default_signed_counter() -> SignedCounter {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_signed_inside_range() {
        assert_eq!(next_signed(0), 1);
        assert_eq!(next_signed(100), 101);
        assert_eq!(next_signed(9223372036854775806), 9223372036854775807);
        assert_eq!(next_signed(-9223372036854775808), -9223372036854775807);
    }

    #[test]
    fn next_signed_out_of_range() {
        assert_eq!(next_signed(isize::MAX - 1).checked_add(2), None);
        assert_eq!(next_signed(isize::MIN + 1).checked_sub(3), None);
    }

    #[test]
    fn next_unsigned_inside_range() {
        assert_eq!(next_unsigned(0), 1);
        assert_eq!(next_unsigned(100), 101);
        assert_eq!(next_unsigned(18446744073709551614), 18446744073709551615);
    }

    #[test]
    fn next_unsigned_out_of_range() {
        assert_eq!(next_unsigned(usize::MAX - 1).checked_add(1), None);
        assert_eq!(next_unsigned(usize::MIN), 1);
    }

    #[test]
    fn prev_signed_inside_range() {
        assert_eq!(prev_signed(1), 0);
        assert_eq!(prev_signed(100), 99);
        assert_eq!(prev_signed(9223372036854775807), 9223372036854775806);
        assert_eq!(prev_signed(-9223372036854775807), -9223372036854775808);
    }

    #[test]
    fn prev_signed_out_of_range() {
        assert_eq!(prev_signed(isize::MAX).checked_add(2), None);
        assert_eq!(prev_signed(isize::MIN + 1).checked_sub(2), None);
    }

    #[test]
    fn default_unsigned_counter_returns_zero() {
        assert_eq!(default_unsigned_counter(), 0);
    }

    #[test]
    fn default_signed_counter_returns_zero() {
        assert_eq!(default_signed_counter(), 0);
    }
}
