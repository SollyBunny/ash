use std::io::{stdin, Error};
use std::os::fd::AsRawFd;

use super::vars;
use super::args;

mod builtins;

pub struct Shell {
	// pub termcur: termios::Termios,
	// pub termold: termios::Termios,
	pub is_run: bool,
	pub is_echo: bool
}

impl Shell {
	pub fn new() -> Result<Shell, Error> {
		// Setup termios
			// let termold = termios::Termios::from_fd(stdin().as_raw_fd())?;
			// let mut termcur = termold.clone();
			// termcur.c_lflag &= !(termios::ICANON | termios::ECHO | termios::ISIG);
			// termcur.c_iflag &= !(termios::BRKINT | termios::ICRNL | termios::INPCK | termios::ISTRIP | termios::IXON);
			// termcur.c_cc[termios::VMIN] = 1;
			// termcur.c_cc[termios::VTIME] = 0;
			// termios::tcsetattr(stdin().as_raw_fd(), termios::TCSANOW, &termcur)?;
		// Setup shell
			let shell = Shell {
				is_run: true,
				is_echo: true
			};
		Ok(shell)
	}
	pub fn close(&mut self) -> Result<(), Error> {
		// termios::tcsetattr(stdin().as_raw_fd(), termios::TCSANOW, &self.termold)?;
		Ok(())
	}
	pub fn eval(&mut self, input: &String) -> Result<String, Error> {
		
		let mut args = args::parse(self, &input)?;
		println!("{} {:?}", input, args);
		if args[0] == "shcall" {
			let builtin = builtins::get(&args[1]);
			if builtin.is_some() {
				args.pop_front();
				return builtin.unwrap()(self, &args);
			}
		} else {
			let var = vars::get(&args[0]);
			if var.is_some() {
				return Ok(var.unwrap());
			} else {
				let mut out = String::new();
				for arg in &args {
					out.push_str(arg);
					out.push(';');
				}
				out.pop();
				return Ok(out);
			}
		}
		Ok(String::new())
	}
}