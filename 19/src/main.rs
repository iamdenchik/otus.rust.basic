use my_macros::filter_even_names;

macro_rules! collect_results {
    ($($func:ident),*) => {
        (
            $( $func() ),*
        )
    };
}

fn foo() -> i32 {
    1
}

fn test_02() -> i32 {
    2
}

fn test_03() -> i32 {
    3
}

fn fo() -> i32 {
    4
}

fn fooo() -> i32 {
    5
}

fn main() {
    let (foo_result, test_02_result, test_03_result) = collect_results!(foo, test_02, test_03);
    println!(
        "Results: {}, {}, {}",
        foo_result, test_02_result, test_03_result
    );

    let result = filter_even_names!("fo", "foo", "fooo");
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use my_macros::filter_even_names;

    #[test]
    fn test_collect_results() {
        let (foo_result, test_02_result, test_03_result) = collect_results!(foo, test_02, test_03);
        assert_eq!((foo_result, test_02_result, test_03_result), (1, 2, 3));
    }

    #[test]
    fn test_filter_even_names() {
        let result = filter_even_names!("fo", "foo", "fooo");
        println!("{:?}", result);
        assert_eq!(result, (4, 5));
    }
}
