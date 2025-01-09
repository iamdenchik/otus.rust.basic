#[derive(Debug, PartialEq)]
pub struct Pair {
    a: i32,
    b: i32,
}

impl Pair {
    pub fn vector_sum(&self, other: &Pair) -> Pair {
        Pair {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }

    pub fn scalar_sum(&self, other: &Pair) -> i32 {
        self.a + self.b + other.a + other.b
    }

    pub fn zero() -> Self {
        Pair { a: 0, b: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_vector_sum_correct_sum_pair() {
        let pair1 = Pair { a: 1, b: 2 };
        let pair2 = Pair { a: 3, b: 3 };
        assert_eq!(pair1.vector_sum(&pair2), Pair { a: 4, b: 5 });
    }

    #[test]
    fn pair_scalar_sum_correct_sum_pair() {
        let pair1 = Pair { a: 1, b: 1 };
        let pair2 = Pair { a: 2, b: 2 };
        assert_eq!(pair1.scalar_sum(&pair2), 6);
    }

    #[test]
    fn default_pair_zero_tuple() {
        let default_pair = Pair::zero();
        assert_eq!(default_pair, Pair { a: 0, b: 0 });
    }
}
