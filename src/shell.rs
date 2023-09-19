use std::io::{Stdin, Stdout, Read, Write, Error};
use std::os::fd::AsRawFd;

use super::vars;

pub struct Shell {
	termin: Stdin,
	termout: Stdout,
	termcur: termios::Termios,
	termold: termios::Termios,
	vars: vars::Vars
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
		// Setup env
			let mut vars = vars::Vars::new();
			vars.set(&"home".to_string(), vars::Var {
				t: vars::Type::Value,
				v: std::env::var("HOME").unwrap()
			});
			vars.set(&"prompt".to_string(), vars::Var {
				t: vars::Type::Value,
				v: "$ ".to_string()
			});
		Ok(Shell {
			termin,
			termout,
			termcur,
			termold,
			vars
		})
	}
	pub fn close(&mut self) -> Result<(), Error> {
		let fdin = self.termin.as_raw_fd();
		termios::tcsetattr(fdin, termios::TCSANOW, &self.termold)?;
		Ok(())
	}
	pub fn eval(&mut self, input: &String) -> Result<String, Error> {
		let varopt = self.vars.get(input);
		if varopt.is_none() {
			return Err(Error::new(std::io::ErrorKind::NotFound, "Variable not found"));
		}
		let var = varopt.unwrap();
		match var.t {
			vars::Type::Value => Ok(var.v.clone()),
			vars::Type::Func => Ok(var.v.clone()),
			vars::Type::Prog => Ok(var.v.clone()),
			_ => Err(Error::new(std::io::ErrorKind::Other, "Invalid type"))
		}
	}
	pub fn prompt(&mut self) -> Result<(), Error> {
		// read byte from self.termin
		let mut inp: String = "".to_string();
		let mut cur: usize = 0; 
		let mut buf: [u8; 1] = [0; 1];
		loop {
			self.termin.read(&mut buf)?;
			match buf[0] {
				b'\n' => break,
				b'\r' => continue,
				b'\x03' | b'\x04' => return Err(Error::new(std::io::ErrorKind::Other, "Interrupted")),
				b'\x7f' => { // backspace
					if cur == 0 { continue }
					cur -= 1;
					inp.remove(cur);
				},
				_ => { // default
					inp.insert(cur, buf[0] as char);
					cur += 1;
				}
			}
			static PROMPT: &str = "prompt";
			let prompt: String = self.eval(&(PROMPT.to_string()))?;
			self.termout.write("\x1b[2K\x1b[1G".as_bytes())?;
			self.termout.write(prompt.as_bytes())?;
			self.termout.write(inp.as_bytes())?;
			self.termout.write("\x1b[".as_bytes())?;
			self.termout.write((cur + prompt.len() + 1).to_string().as_bytes())?;
			self.termout.write("G".as_bytes())?;
			self.termout.flush()?;
		}
		Ok(())
	}
}