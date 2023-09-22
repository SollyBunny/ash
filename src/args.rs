
use std::io::Error;

use super::shell;

#[derive(Debug)] 
pub struct Args {
	pub v: Vec<String>
}

impl Args {
	pub fn new(shell: &mut shell::Shell, string: &String) -> Result<Args, Error> {
		let mut vec: Vec<String> = Vec::new();
		let mut arg: String = String::new();
		let mut isescaped: bool = false;
		let mut isexec: bool = false;
		for (i, c) in string.chars().enumerate() {
			if isescaped {
				match c {
					'0' => arg.push('\0'),
					'n' => arg.push('\n'),
					'r' => arg.push('\r'),
					't' => arg.push('\t'),
					'b' => arg.push('\x08'), // backspace
					'v' => arg.push('\x0b'), // vertical tab
					'f' => arg.push('\x0c'), // form feed
					'a' => arg.push('\x07'), // bell
					'e' => arg.push('\x1b'),
					// TODO: octal, hex and decimal escapes
					_ => arg.push(c)
				}
				isescaped = false;
			} else {
				match c {
					' ' => {
						if arg.len() > 0 {
							vec.push(if isexec {
								shell.eval(&arg)?
							} else {
								arg
							});
							arg = String::new();
						}
					}
					'\\' => {
						isescaped = true;
					}
					'$' => {
						isexec = true;
					}
					_ => {
						arg.push(c);
					}
				}
			}
		}
		if arg.len() > 0 {
			vec.push(if isexec {
				shell.eval(&arg)?
			} else {
				arg
			});
			arg = String::new();
		}
		Ok(Args {
			v: vec
		})
	}
}