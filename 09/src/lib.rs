pub fn task_01(tuple: &mut (i32, i32), a: bool) -> &mut i32 {
    if a {
        &mut tuple.0
    } else {
        &mut tuple.1
    }
}

pub fn task_02(arr: &mut [i32; 3], n: usize) -> &mut i32 {
    &mut arr[n]
}

pub fn task_03(slice: &mut [i32], n: usize) -> &mut i32 {
    &mut slice[slice.len() - n]
}

pub fn task_04(slice: &[i32], n: usize) -> (&[i32], &[i32]) {
    (&slice[..n], &slice[n..])
}

pub fn task_05(slice: &[i32]) -> [&[i32]; 4] {
    let (first_half, second_half) = task_04(slice, slice.len() / 2);
    let (s1, s2) = task_04(first_half, first_half.len() / 2);
    let (s3, s4) = task_04(second_half, second_half.len() / 2);
    [s1, s2, s3, s4]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_01_test_tuple_first() {
        let mut my_tuple = (1, 10);
        assert_eq!(*task_01(&mut my_tuple, true), 1);
    }

    #[test]
    fn task_01_test_tuple_second() {
        let mut my_tuple = (1, 10);
        assert_eq!(*task_01(&mut my_tuple, false), 10);
    }

    #[test]
    fn task_02_test_slice() {
        let mut test_slice = [2, 4, 100];
        assert_eq!(*task_02(&mut test_slice, 2), 100);
    }

    #[test]
    fn task_03_slice_test() {
        let mut t = [2, 4, 100, 3, 5, 6, 7, 8, 9, 10];
        assert_eq!(*task_03(&mut t, 5), 6);
    }

    #[test]
    fn task_04_slice_0() {
        let t = [2, 4, 100, 3, 5, 6, 7, 8, 9, 10];
        assert_eq!(task_04(&t, 5).0, &[2, 4, 100, 3, 5]);
    }

    #[test]
    fn task_04_slice_1() {
        let t = [2, 4, 100, 3, 5, 6, 7, 8, 9, 10];
        assert_eq!(task_04(&t, 5).1, &[6, 7, 8, 9, 10]);
    }

    #[test]
    fn task_05_slice_0() {
        let t = [2, 4, 100, 3, 5, 6, 7, 8, 9, 10];
        let result = task_05(&t);
        assert_eq!(result[0], &[2, 4]);
        assert_eq!(result[1], &[100, 3, 5]);
        assert_eq!(result[2], &[6, 7]);
        assert_eq!(result[3], &[8, 9, 10]);
    }

    #[test]
    fn task_05_slice_1() {
        let empty_slice: &[i32] = &[];
        let result = task_05(empty_slice);
        assert_eq!(result.len(), 4);
        assert_eq!(&result[0], &[]);
        assert_eq!(&result[1], &[]);
        assert_eq!(&result[2], &[]);
        assert_eq!(&result[3], &[]);
    }

    #[test]
    fn task_05_slice_2() {
        let slice: &[i32] = &[2];
        let result = task_05(slice);
        assert_eq!(result.len(), 4);
        assert_eq!(&result[0], &[]);
        assert_eq!(&result[1], &[]);
        assert_eq!(&result[2], &[]);
        assert_eq!(&result[3], &[2]);
    }

    #[test]
    fn task_05_slice_3() {
        let t = [2, 4, 100, 3, 5, 6, 7, 8, 9, 10];
        let result = task_05(&t);
        assert_eq!(result.len(), 4);
    }
}
