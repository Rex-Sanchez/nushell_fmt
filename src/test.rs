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
    assert_eq!(
        format_buffer,
        "if answer == 42 {\n\t\tif pos == 69 {\n\t\t\t\tlet pos = 69\n\t\t}\n}"
    );
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
    assert_eq!(
        format_buffer,
        "http get http://42_is_the_answer.com | from json | list"
    );
}

#[test]
fn forloop() {
    let text = "for i in 0..1 {\n$a = $a + 10\n}";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "for i in 0..1 {\n\t\t$a = $a + 10\n}");
}

#[test]
fn nesting_1() {
    let text = "$item | each {[$in.title $in.text ] } ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "$item | each {[ $in.title $in.text ]}");
}

#[test]
fn nesting_2() {
    let text = "$item | each {\n[$in.title $in.text ] \n} ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(
        format_buffer,
        "$item | each {\n\t\t[ $in.title $in.text ]\n}"
    );
}

#[test]
fn nesting_3() {
    let text = "$item | each {\n{\nd: [$in.title $in.text ],\nb:[$in.title $in.text]\n\t\t}\n} ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "$item | each {\n\t\t{\n\t\t\t\td: [ $in.title $in.text ],\n\t\t\t\tb: [ $in.title $in.text ]\n\t\t}\n}");
}

#[test]
fn nesting_4() {
    let text = "$item | each {\n{\nd: [$in.title [$in.text] ],\nb:[$in.title $in.text]\n\t\t}\n} ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(format_buffer, "$item | each {\n\t\t{\n\t\t\t\td: [ $in.title [ $in.text ]],\n\t\t\t\tb: [ $in.title $in.text ]\n\t\t}\n}");
}

#[test]
fn mismatch_square_braces_1() {
    let text = "$item | each {\n[[$in.title $in.text ] \n} ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(
        format_buffer,
        "$item | each {\n\t\t[[ $in.title $in.text ]\n\t\t}"
    );
}

#[test]
fn mismatch_square_braces_2() {
    let text = "$item | each {\n[ [ { ($in.title $in.text ] \n} ";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(
        format_buffer,
        "$item | each {\n\t\t[[{( $in.title $in.text ]\n\t\t\t\t}"
    );
}

#[test]
fn leading_space() {
    let text = "use ~/.cache/starship/init.nu\n$env.config.buffer_editor = \"nvim\"\n$env.config.buffer_editor = \"nvim\"";
    let format_buffer = format_buffer(text.to_string());
    assert_eq!(
        format_buffer,
        "use ~/.cache/starship/init.nu\n$env.config.buffer_editor = \"nvim\"\n$env.config.buffer_editor = \"nvim\""
    );
}

//#[test]
//fn indent_for_square_brace() {
//    let text = "$a|each{\n[\n{a: a \n}\n]\n}";
//    let format_buffer = format_buffer(text.to_string());
//    assert_eq!(
//        format_buffer,
//        "$a | each {\n\t\t[\n\t\t\t\t{a: a \n\t\t\t\t}\n\t\t]\n}"
//    );
//}

