pub fn extract_numbers(pr_content: String) -> Vec<i128> {
    let numbers: Vec<String> = pr_content.split_whitespace().map(String::from).collect();
    let mut result: Vec<i128> = Vec::new();
    for i in numbers{
        if i.parse::<i128>().is_ok(){
            result.push(i.parse().unwrap());
        }
    }
    result
}