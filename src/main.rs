
mod shell;
mod vars;
mod args;

use std::io::Error;

fn run() -> Result<(), Error> {
	let mut shell = shell::Shell::new()?;
	let mut out: Result<String, Error>;
	static READLINE: &str = "$( $( $(shcall readline) ) )";
	while shell.is_run {
		out = shell.eval(&READLINE.to_string());
		if out.is_err() {
			eprintln!("{:?}", out);
		} else if shell.is_echo {
			println!("{:?}", out);
		}
	}
	shell.close()?;
	Ok(())
}

fn main() {
	let out = run();
	if out.is_err() {
		eprintln!("{:?}", out);
		std::process::exit(1);
	}
}