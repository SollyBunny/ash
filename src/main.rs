

mod shell;
mod vars;
mod args;
mod namespaces;

use std::io::Error;

fn run() -> Result<(), Error> {
	let mut shell = shell::Shell::new()?;
	let mut out: Result<String, Error>;
	let mut msg: String;
	let mut err: Error;
	static READLINE: &str = "readline $prompt";
	loop {
		msg = shell.eval(&READLINE.to_string())?;
		out = shell.eval(&msg);
		if out.is_err() { 
			err = out.unwrap_err();
			if err.kind() == std::io::ErrorKind::Interrupted {
				break
			}
			println!("{}: {}", err.kind().to_string(), err.to_string());
		} else {
			
		};
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