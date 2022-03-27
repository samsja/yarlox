use crate::token;

pub(crate) fn scan_source(source: &String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut start = 0;
    let mut current = 0;
    let mut line = 0;
    let source: Vec<char> = source.chars().collect();

    while current < source.len() {
        let current_token = source[current].to_string();
        println!("{} : {}", current, current_token);
        tokens.push(current_token);
        current += 1;
    }

    tokens
}
