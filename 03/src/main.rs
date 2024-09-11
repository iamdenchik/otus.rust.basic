fn double_int32(numi32: i32) -> i32 {
    numi32 * 2
}

fn double_int64(num32: i32) -> i64 {
    let num64 = num32 as i64;
    num64 * 2
}

fn double_float32(numf32: f32) -> f32 {
    numf32 * 2.0
}

fn double_float64(numf32: f32) -> f64 {
    let numf64 = numf32 as f64;
    numf64 * 2.0
}

fn int_plus_float_to_float(numi32: i32, numf32: f32) -> f64 {
    let numi32_to_f64 = numi32 as f64;
    let numf32_to_f64 = numf32 as f64;
    numi32_to_f64 + numf32_to_f64
}

fn int_plus_float_to_int(numi32: i32, numf32: f32) -> i64 {
    let numi32_to_i64 = numi32 as i64;
    let numf32_to_i64 = numf32 as i64;
    numi32_to_i64 + numf32_to_i64
}

fn tuple_sum(tuple: (i32, i32)) -> i32 {
    tuple.0 + tuple.1
}

fn array_sum(arr: [i32; 3]) -> i32 {
    arr[0] + arr[1] + arr[2]
}

fn main() {
    let double_int32_number = 5;
    let doubled_double_int32 = double_int32(double_int32_number);
    println!(
        "double_int32: передали {} -> получили {}",
        double_int32_number, doubled_double_int32
    );

    let double_int64_number = 6;
    let doubled_double_int64 = double_int64(double_int64_number);
    println!(
        "double_int64: передали {} -> получили {}",
        double_int64_number, doubled_double_int64
    );

    let number_numf32 = 5.5_f32;
    let doubled_numf32 = double_float32(number_numf32);
    println!(
        "double_float32: передали {} -> получили {}",
        number_numf32, doubled_numf32
    );

    let number_double_float64 = 3.1_f32;
    let doubled_double_float64 = double_float64(number_double_float64);
    println!(
        "double_float64: передали {} -> получили {}",
        number_double_float64, doubled_double_float64
    );

    let number_int_plus_float_to_float_numi32 = 6;
    let number_int_plus_float_to_float_numf32 = 2.2_f32;
    let sum_int_plus_float_to_float = int_plus_float_to_float(
        number_int_plus_float_to_float_numi32,
        number_int_plus_float_to_float_numf32,
    );
    println!(
        "int_plus_float_to_float: передали {} и {} -> получили {}",
        number_int_plus_float_to_float_numi32,
        number_int_plus_float_to_float_numf32,
        sum_int_plus_float_to_float
    );

    let number_int_plus_float_to_int_numi32 = 7;
    let number_int_plus_float_to_int_numf32 = 5.9_f32;
    let sum_int_plus_float_to_int = int_plus_float_to_int(
        number_int_plus_float_to_int_numi32,
        number_int_plus_float_to_int_numf32,
    );
    println!(
        "sum_int_plus_float_to_int: передали {} и {} -> получили {}",
        number_int_plus_float_to_int_numi32,
        number_int_plus_float_to_int_numf32,
        sum_int_plus_float_to_int
    );

    let tuple_tuple_sum = (200, 2);
    let sum_tuple_sum = tuple_sum(tuple_tuple_sum);
    println!(
        "tuple_sum: передали {} и {} -> получили сумму {}",
        tuple_tuple_sum.0, tuple_tuple_sum.1, sum_tuple_sum
    );

    let num_array_sum = [2, 4, 100];
    let sum_array_sum = array_sum(num_array_sum);
    println!(
        "array_sum: передали {}, {} и {} -> получили сумму {}",
        num_array_sum[0], num_array_sum[1], num_array_sum[2], sum_array_sum
    );
}
