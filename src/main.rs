

mod shell;
mod vars;

use std::io;
use std::io::Error;

fn run() -> Result<(), Error> {
	let mut shell = shell::Shell::new(io::stdin(), io::stdout())?;
	let mut out: Result<(), Error>;
	loop {
		out = shell.prompt();
		if out.is_err() { break }
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