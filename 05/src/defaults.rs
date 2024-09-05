use crate::types::Pair;
use crate::types::SignedCounter;
use crate::types::UnsignedCounter;
use crate::types::Vec3;

#[cfg(test)]
mod test_default_signed_counter {
    use super::default_signed_counter;

    #[test]
    fn test() {
        assert_eq!(default_signed_counter(), 0);
    }
}

pub fn default_signed_counter() -> SignedCounter {
    0
}

#[cfg(test)]
mod test_default_unsigned_counter {
    use super::default_unsigned_counter;

    #[test]
    fn test() {
        assert_eq!(default_unsigned_counter(), 0);
    }
}

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

#[cfg(test)]
mod test_default_vec3 {
    use super::default_vec3;

    #[test]
    fn test() {
        assert_eq!(default_vec3(), [0; 3]);
    }
}

pub fn default_vec3() -> Vec3 {
    [0; 3]
}

#[cfg(test)]
mod test_default_pair {
    use super::default_pair;

    #[test]
    fn test() {
        assert_eq!(default_pair(), (0, 0));
    }
}

pub fn default_pair() -> Pair {
    (0, 0)
}
