
mod shell;
mod vars;

use std::io::Error;

fn run() -> Result<(), Error> {
	let mut shell = shell::Shell::new()?;
	let mut out: Result<String, Error>;
	shell.eval("shcall setup")?;
	static READLINE: &str = "$(shcall readline)";
	while shell.is_run {
		let readline_var = vars::get("readline");
		out = if readline_var.is_some() {
			shell.eval(readline_var.unwrap())
		} else {
			shell.eval(READLINE)
		};
		if out.is_err() {
			eprintln!("{:?}", out);
		} else if shell.is_echo {
			println!("{}", out.unwrap());
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