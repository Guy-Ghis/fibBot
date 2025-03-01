pub fn extract_numbers(pr_content: &str) -> Vec<i128> {
    let mut pr_numbers = Vec::new();
    let mut found_number = String::new();

    for char in pr_content.chars() {
        if char.is_ascii_digit() {
            found_number.push(char);
        } else if !found_number.is_empty() {
            if let Ok(num) = found_number.parse::<i128>() {
                pr_numbers.push(num);
            }
            found_number.clear();
        }
    }

    if !found_number.is_empty() {
        if let Ok(num) = found_number.parse::<i128>() {
            pr_numbers.push(num);
        }
    }

    pr_numbers
}
