pub struct Counter {
    signed: isize,
    unsigned: usize,
}

impl Counter {
    pub fn next_signed(&mut self) -> isize {
        self.signed + 1
    }

    pub fn next_unsigned(&mut self) -> usize {
        self.unsigned + 1
    }

    pub fn prev_signed(&mut self) -> isize {
        self.signed - 1
    }

    pub fn default_unsigned_counter(&mut self) -> usize {
        0
    }

    pub fn default_signed_counter(&mut self) -> isize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_signed_inside_range() {
        let mut signed_0 = Counter {
            signed: 0,
            unsigned: 0,
        };
        assert_eq!(signed_0.next_signed(), 1);
        let mut signed_100 = Counter {
            signed: 100,
            unsigned: 0,
        };
        assert_eq!(signed_100.next_signed(), 101);
        let mut signed_9223372036854775806 = Counter {
            signed: 9223372036854775806,
            unsigned: 0,
        };
        assert_eq!(
            signed_9223372036854775806.next_signed(),
            9223372036854775807
        );
        let mut signed_minus_9223372036854775808 = Counter {
            signed: -9223372036854775808,
            unsigned: 0,
        };
        assert_eq!(
            signed_minus_9223372036854775808.next_signed(),
            -9223372036854775807
        );
    }

    #[test]
    fn next_signed_out_of_range() {
        let mut signed_max_minus_1 = Counter {
            signed: isize::MAX - 1,
            unsigned: 0,
        };
        assert_eq!(signed_max_minus_1.next_signed().checked_add(2), None);
        let mut signed_min_plus_1 = Counter {
            signed: isize::MIN + 1,
            unsigned: 0,
        };
        assert_eq!(signed_min_plus_1.next_signed().checked_sub(3), None);
    }

    #[test]
    fn next_unsigned_inside_range() {
        let mut unsigned_0 = Counter {
            signed: 0,
            unsigned: 0,
        };
        assert_eq!(unsigned_0.next_unsigned(), 1);
        let mut unsigned_100 = Counter {
            signed: 0,
            unsigned: 100,
        };
        assert_eq!(unsigned_100.next_unsigned(), 101);
        let mut unsigned_18446744073709551614 = Counter {
            signed: 0,
            unsigned: 18446744073709551614,
        };
        assert_eq!(
            unsigned_18446744073709551614.next_unsigned(),
            18446744073709551615
        );
    }

    #[test]
    fn next_unsigned_out_of_range() {
        let mut unsigned_max_minus_1 = Counter {
            signed: 0,
            unsigned: usize::MAX - 1,
        };
        assert_eq!(unsigned_max_minus_1.next_unsigned().checked_add(1), None);
        let mut unsigned_min = Counter {
            signed: 0,
            unsigned: usize::MIN,
        };
        assert_eq!(unsigned_min.next_unsigned(), 1);
    }

    #[test]
    fn prev_signed_inside_range() {
        let mut prev_singned_1 = Counter {
            signed: 1,
            unsigned: 0,
        };
        assert_eq!(prev_singned_1.prev_signed(), 0);
        let mut prev_signed_100 = Counter {
            signed: 100,
            unsigned: 0,
        };
        assert_eq!(prev_signed_100.prev_signed(), 99);
        let mut prev_signed_9223372036854775807 = Counter {
            signed: 9223372036854775807,
            unsigned: 0,
        };
        assert_eq!(
            prev_signed_9223372036854775807.prev_signed(),
            9223372036854775806
        );
        let mut prev_signed_minus_9223372036854775807 = Counter {
            signed: -9223372036854775807,
            unsigned: 0,
        };
        assert_eq!(
            prev_signed_minus_9223372036854775807.prev_signed(),
            -9223372036854775808
        );
    }

    #[test]
    fn prev_signed_out_of_range() {
        let mut prev_signed_max = Counter {
            signed: isize::MAX,
            unsigned: 0,
        };
        assert_eq!(prev_signed_max.prev_signed().checked_add(2), None);
        let mut prev_signed_min_plus_1 = Counter {
            signed: isize::MIN + 1,
            unsigned: 0,
        };
        assert_eq!(prev_signed_min_plus_1.prev_signed().checked_sub(2), None);
    }

    #[test]
    fn default_unsigned_counter_returns_zero() {
        let mut test = Counter {
            signed: 0,
            unsigned: 0,
        };
        assert_eq!(test.default_unsigned_counter(), 0);
    }

    #[test]
    fn default_signed_counter_returns_zero() {
        let mut test = Counter {
            signed: 0,
            unsigned: 0,
        };
        assert_eq!(test.default_signed_counter(), 0);
    }
}
