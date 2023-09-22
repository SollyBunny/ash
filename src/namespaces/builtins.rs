
use std::io::{stdin, stdout, Read, Write, Error};
use std::rc::Rc;

use super::vars;
use super::shell;
use super::args;

fn fn_set(shell: &mut shell::Shell, args: &args::Args) -> Result<String, Error> {
	shell.vars.set(args.get(1), vars::Var::Value(args.get(2).clone()));
	Ok(args.get(2).clone())
}

fn fn_env(shell: &mut shell::Shell, args: &args::Args) -> Result<String, Error> {
	let mut out: String = String::new();
	for (key, var) in &shell.vars.data {
		let new: String = format!("{} {}", key, var);
		out.push_str(new.as_str());
	}
	Ok(out)
}

fn fn_exec(shell: &mut shell::Shell, args: &args::Args) -> Result<String, Error> {
	let mut out: String = String::new();
	for arg in args.v.iter().skip(1) {
		let cmd = std::process::Command::new("sh")
			.arg("-c")
			.arg(arg)
			.output()
		;
		match cmd {
			Ok(msg) => {
				out.push_str(&String::from_utf8_lossy(&msg.stdout));
			}
			Err(msg) => {
				out.push_str(format!("{:?}", msg).as_str());
			}
		}
		
		
	}
	Ok(out)
}

fn fn_readline(shell: &mut shell::Shell, args: &args::Args) -> Result<String, Error> {
	// read byte from shell.termin
	let mut inp: String = "".to_string();
	let mut cur: usize = 0; 
	let mut buf: [u8; 1] = [0; 1];
	let prompt: &String = args.get(1);
	print!("\x1b[2K\x1b[1G{}", prompt);
	stdout().flush()?;
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
	builtins.set("readline", vars::Var::Func(fn_readline));
	builtins.set("env", vars::Var::Func(fn_env));
	builtins.set("set", vars::Var::Func(fn_set));
	builtins.set("exec", vars::Var::Func(fn_exec));
	let namespace = vars::Var::Namespace(Rc::new(builtins));
	shell.vars.set("builtins", namespace);
	shell.vars.importnamespace("builtins")
}