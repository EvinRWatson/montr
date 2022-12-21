pub fn reduce_spaces(mut input: String) -> String {
    while input.contains("  ") {
        input = input.replace("  ", " ");
    }
    return input;
}