pub type Pair = (i32, i32);

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

pub fn default_pair() -> Pair {
    (0, 0)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn pair_vector_sum_correct_sum_pair() {
        assert_eq!(pair_vector_sum((1, 2), (3, 3)), (4, 5));
    }

    #[test]
    fn pair_scalar_sum_correct_sum_pair() {
        assert_eq!(pair_scalar_sum((1, 1), (2, 2)), 6);
    }

    #[test]
    fn default_pair_zero_tuple() {
        assert_eq!(default_pair(), (0, 0));
    }
}
