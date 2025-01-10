use proc_macro::TokenStream;
#[proc_macro]
pub fn filter_even_names(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();
    let function_names: Vec<&str> = input_string
        .split(',')
        .map(|s| s.trim_matches(|c| c == '"' || c == ' '))
        .collect();

    let even_named_functions: Vec<String> = function_names
        .into_iter()
        .filter(|name| name.len() % 2 == 0)
        .map(|name| format!("{}()", name))
        .collect();

    let result = format!("({})", even_named_functions.join(", "));
    result.parse().unwrap()
}
