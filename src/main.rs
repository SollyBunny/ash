

mod shell;
mod vars;
mod builtins;
mod args;

use std::io;
use std::io::{Write, Error};

fn run() -> Result<(), Error> {
	let mut shell = shell::Shell::new(io::stdin(), io::stdout())?;
	let mut out: Result<String, Error>;
	static READLINE: &str = "readline";
	loop {
		out = shell.eval(&READLINE.to_string());
		if out.is_err() { break }
		out = shell.eval(&out.unwrap());
		if out.is_err() { 
			writeln!(shell.termout, "{}", out.unwrap_err())
		} else {
			writeln!(shell.termout, "{}", out.unwrap())
		}?;
	}
	shell.close()?;
	println!("{}", out.unwrap_err());
	Ok(())
}

fn main() {
	let out = run();
	if out.is_err() {
		eprintln!("{:?}", out);
		std::process::exit(1);
	}
}