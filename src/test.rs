use crate::format_buffer;

#[test]
fn remove_leading_and_trailing_whitespace() {
    let text = r#"  let answer = 42  "#;
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, r#"let answer = 42"#);
}

#[test]
fn remove_trailing_newline() {
    let text = "let answer = 42\n";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "let answer = 42");
}

#[test]
fn ignore_comments() {
    let text = "#let answer=42   !";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "#let answer=42   !");
}

#[test]
fn ignore_double_quote_block() {
    let text = "let answer = \"42  \"";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "let answer = \"42  \"");
}

#[test]
fn ignore_single_quote_block() {
    let text = "let answer = '42  '";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "let answer = '42  '");
}

#[test]
fn whitespace_between_words() {
    let text = "let     answer   =     42";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "let answer = 42");
}

#[test]
fn if_indent() {
    let text = "if answer == 42 {\nlet pos = 69\n}";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "if answer == 42 {\n\t\tlet pos = 69\n}");
}

#[test]
fn if_indent_nested() {
    let text = "if answer == 42 {\nif pos == 69 {\nlet pos = 69\n}\n}";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "if answer == 42 {\n\t\tif pos == 69 {\n\t\t\t\tlet pos = 69\n\t\t}\n}");
}
#[test]
fn spacing() {
    let text = "(something=this=that)";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "( something = this = that )");
}
#[test]
fn not_eq() {
    let text = "if a != 42 {\n$pos = 69\n} ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "if a != 42 {\n\t\t$pos = 69\n}");
}
#[test]
fn more_eq() {
    let text = "if a >= 42 {\n$pos = 69\n} ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "if a >= 42 {\n\t\t$pos = 69\n}");
}
#[test]
fn less_eq() {
    let text = "if a <= 42 {\n$pos = 69\n} ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "if a <= 42 {\n\t\t$pos = 69\n}");
}
#[test]
fn less() {
    let text = "if a < 42 {\n$pos = 69\n} ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "if a < 42 {\n\t\t$pos = 69\n}");
}
#[test]
fn more() {
    let text = "if a > 42 {\n$pos = 69\n} ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "if a > 42 {\n\t\t$pos = 69\n}");
}
#[test]
fn to_many_curlys() {
    let text = "if a > 42 {\n$pos = 69\n}\n}let a = 42";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "if a > 42 {\n\t\t$pos = 69\n}\n} let a = 42");
}
#[test]
fn to_little_curlys() {
    let text = "if a > 42 {\n$pos = 69\nlet a = 42";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "if a > 42 {\n\t\t$pos = 69\n\t\tlet a = 42");
}

 #[test]
fn path_non_trailing_whitespace() {
    let text = "/home/user/folder/text.lua";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "/home/user/folder/text.lua");
}

 #[test]
fn path_with_trailing_whitespace() {
    let text = "/home/user/folder/text.lua ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "/home/user/folder/text.lua");
}

 #[test]
fn pipe_1() {
    let text = "http get http://42_is_the_answer.com |from json|list ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "http get http://42_is_the_answer.com | from json | list");
}
