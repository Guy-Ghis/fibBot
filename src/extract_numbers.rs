// pub fn extract_numbers(pr_content: &str) -> Vec<i128> {
//     let mut pr_numbers = Vec::new();
//     let mut found_number = String::new();

//     for char in pr_content.chars() {
//         if char.is_ascii_digit() {
//             found_number.push(char);
//         } else if !found_number.is_empty() {
//             if let Ok(num) = found_number.parse::<i128>() {
//                 pr_numbers.push(num);
//             }
//             found_number.clear();
//         }
//     }

//     if !found_number.is_empty() {
//         if let Ok(num) = found_number.parse::<i128>() {
//             pr_numbers.push(num);
//         }
//     }

//     pr_numbers
// }

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

    // Check if there's a number left at the end of the string
    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<u32>() {
            numbers.push(num.try_into().unwrap());
        }
    }

    numbers
}