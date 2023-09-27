
use super::shell;

use std::collections::VecDeque;
use std::io::Error;

pub type Args = VecDeque<String>;

fn chec_end(args: &mut Args) {
}

pub fn parse(shell: &mut shell::Shell, input: &String) -> Result<Args, Error> {
	let mut args: Args = Args::new();
	let mut isescaped: bool = false;
	let mut iseval: bool = false;
	let mut brackets = 0;
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
							if brackets > 0 {
								println!("{} {} '{}'", c, brackets, arg);
								arg.push(' ');
							} else {
								// arg = format!("({})", arg);
								println!("{}", arg);
								args.push_back(shell.eval(&arg)?);
								arg = String::new();
							}
						} else {
							args.push_back(arg);
							arg = String::new();
						}
					}
				}
				'\\' => {
					isescaped = true;
				}
				'$' => {
					if iseval { continue; }
					iseval = true;
					brackets = 0;
				}
				'(' => {
					brackets += 1;
				}
				')' => {
					brackets -= 1;
				}
				_ => {
					arg.push(c);
				}
			}
		}
	}
	if arg.len() > 0 {
		if arg.len() > 0 {
			if iseval {
				if brackets > 0 { return Ok(args); }
				args.push_back(shell.eval(&arg)?);
			} else {
				args.push_back(arg);
			}
			arg = String::new();
		}
	}
	Ok(args)
}