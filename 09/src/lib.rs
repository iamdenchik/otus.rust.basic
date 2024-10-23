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

pub fn task_05(slice: &[i32]) -> (&[i32], &[i32], &[i32], &[i32]) {
    let a = task_04(slice, slice.len() / 2);
    let b = task_04(a.0, a.0.len() / 2);
    let c = task_04(a.1, a.1.len() / 2);
    (b.0, b.1, c.0, c.1)
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
        let my_slice = &t;
        assert_eq!(task_04(my_slice, 5).0, &[2, 4, 100, 3, 5]);
    }

    #[test]
    fn task_04_slice_1() {
        let t = [2, 4, 100, 3, 5, 6, 7, 8, 9, 10];
        let my_slice = &t;
        assert_eq!(task_04(my_slice, 5).1, &[6, 7, 8, 9, 10]);
    }

    #[test]
    fn task_05_slice_0() {
        let t = [2, 4, 100, 3, 5, 6, 7, 8, 9, 10];
        let my_slice = &t;
        assert_eq!(task_05(my_slice).0, &[2, 4]);
    }

    #[test]
    fn task_05_slice_1() {
        let t = [2, 4, 100, 3, 5, 6, 7, 8, 9, 10];
        let my_slice = &t;
        assert_eq!(task_05(my_slice).1, &[100, 3, 5]);
    }

    #[test]
    fn task_05_slice_2() {
        let t = [2, 4, 100, 3, 5, 6, 7, 8, 9, 10];
        let my_slice = &t;
        assert_eq!(task_05(my_slice).2, &[6, 7]);
    }

    #[test]
    fn task_05_slice_3() {
        let t = [2, 4, 100, 3, 5, 6, 7, 8, 9, 10];
        let my_slice = &t;
        assert_eq!(task_05(my_slice).3, &[8, 9, 10]);
    }
}
