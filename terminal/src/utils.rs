use crate::prelude::*;
use std::fmt::Display;



pub fn clear() {
	print!("{}c", 27 as char);
}

pub fn wait() {
	println!();
	println!("Press enter to continue");
	read!();
}

pub fn wait_and_clear() {
	wait();
	clear();
}



pub fn pluralize<'a>(count: f32, singular: &'a str, plural: &'a str) -> &'a str {
	if count == 1. {singular} else {plural}
}

pub fn stringify_list(list: &[impl Display]) -> String {
	match list.len() {
		0 => String::new(),
		1 => format!("{}", list[0]),
		2 => format!("{} and {}", list[0], list[1]),
		len => {
			let mut output = format!("{}", list[0]);
			for item in &list[1..len-1] {
				output += &format!(", {item}");
			}
			output += &format!(", and {}", list[len-1]);
			output
		}
	}
}



pub fn some_if<T>(input: T, condition: bool) -> Option<T> {
	if condition {Some(input)} else {None}
}
