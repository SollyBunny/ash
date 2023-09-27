
use std::collections::HashMap;
use std::io::{stderr, Write, Error};

use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
	event::{read, Event, KeyCode, KeyModifiers}
};


use super::{Shell, Args, vars};

pub type BuiltinType = fn(shell: &mut Shell, args: &Args) -> Result<String, Error>;

static mut BUILTINS: Option<HashMap<&'static str, BuiltinType>> = None;

fn fn_readline(shell: &mut Shell, _args: &Args) -> Result<String, Error> {
	let mut inp: String = "".to_string();
	let mut cur: usize = 0; 
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
			cursor::MoveTo(initial_x + cur as u16, initial_y),
		)?;
		match read()? {
			// Event::FocusGained => println!("FocusGained"),
			// Event::FocusLost => println!("FocusLost"),
			// Event::Mouse(event) => println!("{:?}", event),
			// #[cfg(feature = "bracketed-paste")]
			// Event::Paste(data) => println!("Pasted {:?}", data),
			// Event::Resize(width, height) => println!("New size {}x{}", width, height),
            Event::Key(event) => {
				if event.modifiers == KeyModifiers::SHIFT || event.modifiers == KeyModifiers::NONE {
					match event.code {
						KeyCode::Backspace => {
							if cur == 0 { continue }
							cur -= 1;
							inp.remove(cur);
						}
						KeyCode::Delete => {
							if inp.len() == 0 { continue }
							if cur == inp.len() {
								cur -= 1;
								inp.remove(cur);
							} else {
								inp.remove(cur + 1);
							}
						}
						KeyCode::Left => {
							if cur == 0 { continue }
							cur -= 1;
						}
						KeyCode::Right => {
							if cur == inp.len() { continue }
							cur += 1;
						}
						KeyCode::Up => {
							
						}
						KeyCode::Home => {
							cur = 0;
						}
						KeyCode::End => {
							cur = inp.len();
						}
						KeyCode::Enter => {
							break
						}
						KeyCode::Char(c) => {
							inp.insert(cur, c);
							cur += 1;
						}
						_ => {}
					}
				} else if event.modifiers == KeyModifiers::CONTROL {
					match event.code {
						KeyCode::Backspace => { // Delete last word
							
						}
						KeyCode::Left => {
							
						}
						KeyCode::Right => {
							
						}
						KeyCode::Char(c) => {
							match c {
								'c' => {
									inp.clear();
									break;
								}
								'd' => {
									if inp.is_empty() {
										shell.is_run = false;
									} else {
										inp.clear();
									}
									break;
								}
								'q' => {
									shell.is_run = false;
									break;
								}
								_ => {}
							}
						}
						_ => {}
					}
				}
			}
			_ => {}
		}
	}
	terminal::disable_raw_mode()?;
	write!(stderr(), "\n")?;
	Ok(inp)
}

fn fn_exec(_shell: &mut Shell, args: &Args) -> Result<String, Error> {
	let mut out: String = String::new();
	for arg in args.iter().skip(1) {
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

fn fn_set(_shell: &mut Shell, args: &Args) -> Result<String, Error> {
	Ok(match args.len() {
		1 => {
			String::new()
		}
		2 => {
			vars::del(&args[1]);
			String::new()
		}
		_ => {
			vars::set(&args[1], &args[2]);
			args[2].clone()
		}
	})
}

pub fn fn_echo (_shell: &mut Shell, args: &Args) -> Result<String, Error> {
	let mut out = String::new();
	for arg in args.iter().skip(1) {
		out.push_str(&arg);
		out.push_str(" ");
	}
	out.pop();
	println!("{}", out);
	Ok(out)
}

pub fn fn_setup(_shell: &mut Shell, _args: &Args) -> Result<String, Error> { unsafe {
	for (key, _) in BUILTINS.as_ref().unwrap().iter() {
		vars::set(key, format!("shcall {}", key));
	}
	vars::del("readline");
	// vars::set("readline", r"(echo >\ ;shcall readline)");
	Ok(String::new())
} }

pub fn get<T: AsRef<str>>(ref key: T) -> Option<&'static BuiltinType> { unsafe {
	if BUILTINS.is_none() {
		let mut hashmap: HashMap<&'static str, BuiltinType> = HashMap::new();
		hashmap.insert("readline", fn_readline);
		hashmap.insert("exec", fn_exec);
		hashmap.insert("set", fn_set);
		hashmap.insert("echo", fn_echo);
		hashmap.insert("setup", fn_setup);
		BUILTINS = Some(hashmap);
	}
	BUILTINS.as_ref().unwrap().get(key.as_ref())
} }