use std::io::{Error, ErrorKind};
use std::collections::VecDeque;

use super::vars;

pub type Args = VecDeque<String>;

mod builtins;

pub struct Shell {
	pub is_run: bool,
	pub is_echo: bool,
	pub history: Vec<String>
}

impl Shell {
	pub fn new() -> Result<Shell, Error> {
		// Setup shell
			let shell = Shell {
				is_run: true,
				is_echo: true,
				history: Vec::with_capacity(100)
			};
		Ok(shell)
	}
	pub fn close(&mut self) -> Result<(), Error> {
		// termios::tcsetattr(stdin().as_raw_fd(), termios::TCSANOW, &self.termold)?;
		Ok(())
	}
	pub fn eval<T: AsRef<str>>(&mut self, input: T) -> Result<String, Error> {
		self.eval_raw(input, 0)
	}
	pub fn eval_raw<T: AsRef<str>>(&mut self, input: T, depth: u32) -> Result<String, Error> {
		if depth > 64 {
			return Err(Error::new(ErrorKind::Other, "Recursion limit exceeded"));
		}
		let mut args: Args = Args::new();
		let mut isescaped: bool = false;
		let mut iseval: bool = false;
		let mut brackets = 0;
		let mut iscontinue = false;
		let mut from = 0;
		let mut arg: String = String::new();
		for (i, c) in input.as_ref().chars().enumerate() {
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
			} else if iseval {
				match c {
					' ' => {
						if arg.len() > 0 && brackets == 0 {
							let out = arg.trim_matches('$').trim_matches('(').trim_matches(')');
							// let out = self.eval_raw(out, depth + 1)?;
							args.push_back(self.eval_raw(out, depth + 1)?);
							arg = String::new();
							iseval = false;
						} else {
							arg.push(c);
						}
					}
					'(' => {
						brackets += 1;
						arg.push(c);
					}
					')' => {
						brackets -= 1;
						arg.push(c);
					}
					_ => {
						arg.push(c);
					}
				}
			} else {
				match c {
					' ' => {
						if arg.len() > 0 {
							args.push_back(arg);
							arg = String::new();
						}
					}
					'\\' => {
						isescaped = true;
					}
					'$' => {
						iseval = true;
						brackets = 0;
					}
					';' | '\n' => {
						iscontinue = true;
						from = i;
						break;
					}
					_ => {
						arg.push(c);
					}
				}
			}
		}
		if arg.len() > 0 {
			if iseval {
				if brackets >= 0 {
					let stripped: &str = arg.trim_matches('$').trim_matches('(').trim_matches(')');
					let out = self.eval_raw(stripped, depth + 1)?;
					args.push_back(self.eval_raw(out, depth + 1)?);
				}
			} else {
				args.push_back(arg);
			}
		}
		// println!("{}: \"{}\" -> {:?}", depth + 1, input.as_ref(), args);
		let mut out: String;
		if args.len() == 0 {
			out = String::new();
		} else if args[0] == "shcall" {
			if args.len() == 1 {
				out = String::new();
			} else {
				let builtin = builtins::get(&args[1]);
				if builtin.is_some() {
					args.pop_front();
					out = builtin.unwrap()(self, &args)?;
				} else {
					out = String::new();
				}
			}
		} else {
			let var = vars::get(&args[0]);
			if var.is_some() {
				out = var.unwrap();
			} else {
				out = String::new();
				for arg in &args {
					out.push_str(arg);
					out.push(';');
				}
				out.pop();
			}
		}
		if iscontinue {
			out = self.eval_raw(&input.as_ref()[from + 1..], depth + 1)?;
		}
		Ok(out)
	}
}