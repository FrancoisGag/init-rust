use std::io::stdin;

fn what_is_your_name() -> String {
  let mut your_name = String::new();
	stdin()
		.read_line(&mut your_name)
		.expect("Failed to read line");

	your_name.trim().to_lowercase()
}

fn main() {
  println!("Hello, what's your name?");
  let name = what_is_your_name();
	let visitor_list = ["bert", "bart", "kak"];
	let mut allowed = false;
	for visitor in &visitor_list {
		if &name == visitor {
			allowed = true;
			break;
		}
	}

	if allowed {
		println!("Hello, {}", name);     
	} else {
		println!("Sorry, you're not on the list");     
	}
}
