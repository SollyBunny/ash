

mod shell;
mod vars;
mod builtins;
mod args;

use std::io;
use std::io::Error;

fn run() -> Result<(), Error> {
	let mut shell = shell::Shell::new(io::stdin(), io::stdout())?;
	let mut out: Result<String, Error>;
	static READLINE: &str = "readline";
	loop {
		out = shell.eval(&READLINE.to_string());
		if out.is_err() { break }
		shell.eval(&out.unwrap());
	}
	shell.close()?;
	println!("{}", out.unwrap_err());
	Ok(())
}

fn main() {
	let out = run();
	if out.is_err() {
		eprintln!("{}", out.unwrap_err());
		std::process::exit(1);
	}
}