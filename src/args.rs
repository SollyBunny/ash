
use std::io::Error;

use super::shell;

pub struct Args {
	pub v: Vec<String>,
	emptystring: String
}

impl Args {
	pub fn new(shell: &mut shell::Shell, input: &String) -> Result<Args, Error> {
		let mut args: Vec<String> = Vec::new();
		let mut isescaped: bool = false;
		let mut iseval: bool = false;
		let mut arg: String = String::new();
		for c in input.chars() {
			if isescaped {
				arg.push(match c {
					'0' => '\0',
					't' => '\t',
					'n' => '\n',
					'r' => '\r',
					'e' => '\x1b',
					'a' => '\x07', // Terminal Bell
					'b' => '\x08', // Backspace
					'v' => '\x0b', // Vertical Tab
					'f' => '\x0c', // New Page
					// TODO: allow oct, dec, hex and unicode escapes
					_ => c
				});
			} else {
				match c {
					' ' => {
						if arg.len() > 0 {
							if iseval {
								args.push(shell.eval(&arg)?)
							} else {
								args.push(arg)
							}
							arg = String::new();
							iseval = false;
							isescaped = false;
						}
					}
					'$' => {
						iseval = true;
					}
					'\\' => {
						isescaped = true;
					}
					_ => {
						arg.push(c);
					}
				}
			}
		}
		if arg.len() > 0 {
			if iseval {
				args.push(shell.eval(&arg)?)
			} else {
				args.push(arg)
			}
		}
		Ok(Args {
			v: args,
			emptystring: String::new()
		})
	}
	pub fn get(&self, index: usize) -> &String {
		if index < self.v.len() {
			&self.v[index]
		} else {
			&self.emptystring
		}
	}
}