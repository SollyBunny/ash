use std::io::{Stdin, Stdout, Error};
use std::os::fd::AsRawFd;

use super::vars;
use super::args;
use super::builtins;

pub struct Shell {
	pub termin: Stdin,
	pub termout: Stdout,
	pub termcur: termios::Termios,
	pub termold: termios::Termios,
	pub vars: vars::Vars
}

impl Shell {
	pub fn new(termin: Stdin, termout: Stdout) -> Result<Shell, Error> {
		// Setup termios
			let fdin = termin.as_raw_fd();
			let termold = termios::Termios::from_fd(fdin)?;
			let mut termcur = termios::Termios::from_fd(fdin)?;
			termcur.c_lflag &= !(termios::ICANON | termios::ECHO | termios::ISIG);
			termcur.c_iflag &= !(termios::BRKINT | termios::ICRNL | termios::INPCK | termios::ISTRIP | termios::IXON);
			termcur.c_cc[termios::VMIN] = 1;
			termcur.c_cc[termios::VTIME] = 0;
			termios::tcsetattr(fdin, termios::TCSANOW, &termcur)?;
		// Setup shell
			let vars = vars::Vars::new();
			let mut shell = Shell {
				termin,
				termout,
				termcur,
				termold,
				vars
			};
			builtins::add(&mut shell)?;
		Ok(shell)
	}
	pub fn close(&mut self) -> Result<(), Error> {
		let fdin = self.termin.as_raw_fd();
		termios::tcsetattr(fdin, termios::TCSANOW, &self.termold)?;
		Ok(())
	}
	pub fn eval(&mut self, input: &String) -> Result<String, Error> {
		let var = &**self.vars.get(input)?;
		match var {
			vars::Var::Value(v) => Ok(v.clone()),
			vars::Var::Func(f) => {
				Ok(f(self, &args::Args::new())?)
			},
			vars::Var::Namespace(_n) => Ok("Not Impled Yet".to_string())
		}
	}
}