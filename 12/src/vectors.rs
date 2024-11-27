pub const VEC3_LEN: usize = 3;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    x: [i32; VEC3_LEN],
}

impl Vec3 {
    pub fn vector_sum(&self, a: &Vec3) -> Vec3 {
        let mut c = [0; VEC3_LEN];
        for i in 0..3 {
            c[i] = self.x[i] + a.x[i];
        }
        Vec3 { x: c }
    }

    pub fn scalar_sum(&self, b: Vec3) -> i32 {
        let mut c = 0;
        for i in 0..VEC3_LEN {
            c += self.x[i] + b.x[i];
        }
        c
    }

    pub fn default() -> Self {
        Vec3 { x: [0; VEC3_LEN] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_vector_sum_correct_sum_tuple() {
        let vec1 = Vec3 { x: [1; 3] };
        let vec2 = Vec3 { x: [2; 3] };
        assert_eq!(vec1.vector_sum(&vec2).x, [3, 3, 3]);
        let vec3 = Vec3 { x: [1; 3] };
        assert_ne!(vec3.vector_sum(&vec3).x, [3, 3, 3]);
    }

    #[test]
    fn vec3_scalar_sum_correct_sum_tuple() {
        let vec1 = Vec3 { x: [1, 2, 3] };
        let vec2 = Vec3 { x: [1, 2, 3] };
        assert_eq!(vec1.scalar_sum(vec2), 12);
    }

    #[test]
    fn default_vec3_zero_tuple() {
        let default_vec3 = Vec3::default();
        assert_eq!(default_vec3, Vec3 { x: [0; 3] });
    }
}
