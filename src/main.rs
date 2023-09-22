

mod shell;
mod vars;
mod args;
mod namespaces;

use std::io::Error;

fn run() -> Result<(), Error> {
	let mut shell = shell::Shell::new()?;
	let mut out: Result<String, Error>;
	static READLINE: &str = "readline $prompt";
	loop {
		out = shell.eval(&READLINE.to_string());
		if out.is_err() { break; }
		out = shell.eval(&out.unwrap());
		if out.is_err() { 
			println!("{}", out.unwrap_err());
		} else {
			println!("{}", out.unwrap());
		}
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