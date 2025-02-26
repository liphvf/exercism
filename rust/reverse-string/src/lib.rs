pub fn reverse(input: &str) -> String {
    let mut result = String::new();

    let reverse_input = input.chars().rev();
    
    for i in reverse_input {
        result.push(i);
    }

    return result;
}
