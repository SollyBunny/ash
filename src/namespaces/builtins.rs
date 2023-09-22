
use std::io::{stdin, stdout, Read, Write, Error};
use std::rc::Rc;

use super::vars;
use super::shell;
use super::args;

fn fn_set(shell: &mut shell::Shell, args: &args::Args) -> Result<String, Error> {
	Ok("".to_string())
}

fn fn_readline(shell: &mut shell::Shell, args: &args::Args) -> Result<String, Error> {
	// read byte from shell.termin
	let mut inp: String = "".to_string();
	let mut cur: usize = 0; 
	let mut buf: [u8; 1] = [0; 1];
	let prompt: &String = args.get(1);
	loop {
		stdin().read(&mut buf)?;
		match buf[0] {
			b'\r' | b'\n' => break,
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
		print!("\x1b[2K\x1b[1G{}{}\x1b[{}G", prompt, inp, (cur + prompt.len() + 1));
		stdout().flush()?;
	}
	println!();
	Ok(inp)
}

pub fn add(shell: &mut shell::Shell) -> Result<(), Error> {
	let mut builtins = vars::Vars::new();
	builtins.set("home", vars::Var::Value(
		std::env::var("HOME").unwrap()
	));
	builtins.set("prompt", vars::Var::Value(
		"$ ".to_string()
	));
	builtins.set(&"readline".to_string(), vars::Var::Func(
		fn_readline
	));
	let namespace = vars::Var::Namespace(Rc::new(builtins));
	shell.vars.set("builtins", namespace);
	shell.vars.importnamespace("builtins")
}