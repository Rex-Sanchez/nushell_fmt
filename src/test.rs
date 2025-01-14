use crate::format_buffer;

#[test]
fn remove_leading_and_trailing_whitespace() {
    let text = r#"  let answer = 42  "#;
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, r#"let answer = 42"#);
}

//#[test]
//fn remove_trailing_newline() {
//    let text = "let answer = 42\n";
//    let format_buffer = format_buffer(text.to_string());
//    assert_eq!(format_buffer, "let answer = 42");
//}

#[test]
fn ignore_comments() {
    let text = "#let answer=42   !";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "#let answer=42   !");
}
