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
