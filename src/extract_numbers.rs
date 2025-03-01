pub fn extract_numbers(pr_content: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let mut current_number = String::new();

    for c in pr_content.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else if !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<u32>() {
                numbers.push(num);
            }
            current_number.clear();
        }
    }

    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<u32>() {
            numbers.push(num.try_into().unwrap());
        }
    }

    numbers
}