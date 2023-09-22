use std::io::{stdin, Error};
use std::os::fd::AsRawFd;

use super::vars;
use super::args;
use super::namespaces;

pub struct Shell {
	pub termcur: termios::Termios,
	pub termold: termios::Termios,
	pub vars: vars::Vars
}

impl Shell {
	pub fn new() -> Result<Shell, Error> {
		// Setup termios
			let termold = termios::Termios::from_fd(stdin().as_raw_fd())?;
			let mut termcur = termios::Termios::from_fd(stdin().as_raw_fd())?;
			termcur.c_lflag &= !(termios::ICANON | termios::ECHO | termios::ISIG);
			termcur.c_iflag &= !(termios::BRKINT | termios::ICRNL | termios::INPCK | termios::ISTRIP | termios::IXON);
			termcur.c_cc[termios::VMIN] = 1;
			termcur.c_cc[termios::VTIME] = 0;
			termios::tcsetattr(stdin().as_raw_fd(), termios::TCSANOW, &termcur)?;
		// Setup shell
			let vars = vars::Vars::new();
			let mut shell = Shell {
				termcur,
				termold,
				vars
			};
			namespaces::add(&mut shell)?;
		Ok(shell)
	}
	pub fn close(&mut self) -> Result<(), Error> {
		termios::tcsetattr(stdin().as_raw_fd(), termios::TCSANOW, &self.termold)?;
		Ok(())
	}
	pub fn eval(&mut self, input: &String) -> Result<String, Error> {
		let args = args::Args::new(self, input)?;
		let var = &**self.vars.get(args.get(0))?;
		match var {
			vars::Var::Value(v) => Ok(v.clone()),
			vars::Var::Func(f) => {
				f(self, &args)
			},
			vars::Var::Namespace(_n) => Ok("Not Impled Yet".to_string())
		}
	}
}