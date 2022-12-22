pub fn reduce_spaces(mut input: String) -> String {
    while input.contains("  ") {
        input = input.replace("  ", " ");
    }
    return input;
}

pub fn convert_from_gib_to_gb(input: String) -> String {
    let raw_value: String = input.replace("Gi", "");
    let output_value: f32 = raw_value.parse::<f32>().unwrap() * 1.073741824;
    return format!("{}GB", output_value.to_string());
}