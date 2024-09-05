use crate::constants::VEC3_LEN;
use crate::defaults::default_vec3;
use crate::types::Pair;
use crate::types::Vec3;

#[cfg(test)]
mod test_vec3_vector_sum {
    use super::vec3_vector_sum;

    #[test]
    fn test() {
        assert_eq!(vec3_vector_sum([1; 3], [2; 3]), [3, 3, 3]);
        assert_ne!(vec3_vector_sum([1; 3], [1; 3]), [3, 3, 3]);
    }
}

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod test_pair_vector_sum {
    use super::pair_vector_sum;

    #[test]
    fn test() {
        assert_eq!(pair_vector_sum((1, 2), (3, 3)), (4, 5));
    }
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

#[cfg(test)]
mod test_vec3_scalar_sum {
    use super::vec3_scalar_sum;

    #[test]
    fn test() {
        assert_eq!(vec3_scalar_sum([1, 2, 3], [1, 2, 3]), 12);
    }
}

pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod test_pair_scalar_sum {
    use super::pair_scalar_sum;

    #[test]
    fn test() {
        assert_eq!(pair_scalar_sum((1, 1), (2, 2)), 6);
    }
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}
