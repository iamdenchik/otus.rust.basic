pub const VEC3_LEN: usize = 3;
pub type Vec3 = [i32; VEC3_LEN];

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

pub fn default_vec3() -> Vec3 {
    [0; 3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_vector_sum_correct_sum_tuple() {
        assert_eq!(vec3_vector_sum([1; 3], [2; 3]), [3, 3, 3]);
        assert_ne!(vec3_vector_sum([1; 3], [1; 3]), [3, 3, 3]);
    }

    #[test]
    fn vec3_scalar_sum_correct_sum_tuple() {
        assert_eq!(vec3_scalar_sum([1, 2, 3], [1, 2, 3]), 12);
    }

    #[test]
    fn default_vec3_zero_tuple() {
        assert_eq!(default_vec3(), [0; 3]);
    }
}
