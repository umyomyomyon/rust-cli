pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap(); // writeln! returns Result<(), Err>
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::<u8>::new(); // Vec<u8> is implemented std::io::Write.
    find_matches("hoge\nhuga\npiyo\n", "hoge", &mut result);
    assert_eq!(result, b"hoge\n");
}

pub fn validate_pattern(pattern: &str) -> Result<(), &str> {
    if pattern.is_empty() {
        return Err("please input pattern")
    }
    Ok(())
}

#[test]
fn validate_empty_pattern() {
    match validate_pattern("") {
        Ok(_) => (),
        Err(err) => assert_eq!(err, "please input pattern")
    }
}

#[test]
fn validate_pattern_not_empty() {
    match validate_pattern("not empty") {
        Ok(ok) => assert_eq!(ok, ()),
        Err(_) => ()
    }
}
