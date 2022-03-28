pub fn error(line: usize, message: &str) {
    report(line, &"".to_string(), &message);
}

pub fn report(line: usize, _where: &str, message: &str) {
    println!("[line {}] Error {} : {}", line, _where, message);
}
