#[test]
fn format_test() {
let text = r#"
def pm [ ] {
	mut locations = [ item, item2, item3 ]
	let    selection = abs | cselect | as | as | as
	   let a = "this is |  should not touch | | | | "
if $selection != null {
		let a = foobar
		let c = bar
		let b = a | each { | item | item | do_somehting }
		mpv $selection
	}
	
  let a = glob (/home/n3m0/AnyDesk/**/*.{as,as,as})
    let b = sssss
} "#;


//let format_buffer = crate::format_buffer(text.to_string());

//println!("{}",format_buffer);









}
#[test]
fn format_test1() {
let text = r#"
def pm [ ] {
    let b = $"nlslslsls"
} "#;


let format_buffer = crate::format_buffer(text.to_string());

println!("{}",format_buffer);









}
