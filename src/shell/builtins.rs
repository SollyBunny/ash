
use std::collections::HashMap;
use std::io::{stderr, stdin, Read, Write, Error};

use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};

use super::Shell;
use super::args;

pub type BuiltinType = fn(shell: &mut Shell, args: &args::Args) -> Result<String, Error>;

static mut BUILTINS: Option<HashMap<&'static str, BuiltinType>> = None;

fn fn_readline(shell: &mut Shell, _args: &args::Args) -> Result<String, Error> {
	let mut inp: String = "".to_string();
	let mut cur: u16 = 0; 
	let mut buf: [u8; 1] = [0; 1];
	terminal::enable_raw_mode()?;
	let (initial_x, initial_y) = cursor::position()?;
	loop {
		execute!(
			std::io::stderr(),
			cursor::MoveTo(initial_x, initial_y),
			terminal::Clear(ClearType::UntilNewLine),
		)?;
		write!(stderr(), "{}", inp)?;
		execute!(
			std::io::stderr(),
			cursor::MoveTo(initial_x + cur, initial_y),
		)?;
		stdin().read(&mut buf)?;
		match buf[0] {
			b'\r' | b'\n' => break,
			b'\x03' => {
				inp.clear();
				break;
			}
			b'\x04' => {
				if inp.is_empty() {
					shell.is_run = false;
				} else {
					inp.clear();
				}
				break;
			}
			b'\x7f' => { // backspace
				if cur == 0 { continue }
				cur -= 1;
				inp.remove(cur.into());
			},
			_ => { // default
				inp.insert(cur.into(), buf[0] as char);
				cur += 1;
			}
		}
	}
	terminal::disable_raw_mode()?;
	write!(stderr(), "\n")?;
	Ok(inp)
}

pub fn get<T: AsRef<str>>(ref key: T) -> Option<&'static BuiltinType> { unsafe {
	if BUILTINS.is_none() {
		let mut hashmap: HashMap<&'static str, BuiltinType> = HashMap::new();
		hashmap.insert("readline", fn_readline);
		BUILTINS = Some(hashmap);
	}
	BUILTINS.as_ref().unwrap().get(key.as_ref())
} }