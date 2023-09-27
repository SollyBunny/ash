
// use super::shell;

// use std::io::Error;



// fn chec_end(args: &mut Args) {
// }

// pub fn parse(shell: &mut shell::Shell, input: &String) -> Result<Args, Error> {
// 	let mut args: Args = Args::new();
// 	let mut isescaped: bool = false;
// 	let mut iseval: bool = false;
// 	let mut brackets = 0;
// 	let mut arg: String = String::new();
// 	for c in input.as_ref().chars() {
// 		if isescaped {
// 			arg.push(match c {
// 				'0' => '\0',
// 				't' => '\t',
// 				'n' => '\n',
// 				'r' => '\r',
// 				'e' => '\x1b',
// 				'a' => '\x07', // Terminal Bell
// 				'b' => '\x08', // Backspace
// 				'v' => '\x0b', // Vertical Tab
// 				'f' => '\x0c', // New Page
// 				// TODO: allow oct, dec, hex and unicode escapes
// 				_ => c
// 			});
// 		} else if iseval {
// 			match c {
// 				' ' => {
// 					if arg.len() > 0 && brackets == 0 {
// 						let stripped: &str;
// 						stripped = arg.trim_matches('$').trim_matches('(').trim_matches(')');
// 						args.push_back(shell.eval_raw(stripped, depth + 1)?);
// 						arg = String::new();
// 						iseval = false;
// 					} else {
// 						arg.push(c);
// 					}
// 				}
// 				'(' => {
// 					brackets += 1;
// 					arg.push(c);
// 				}
// 				')' => {
// 					brackets -= 1;
// 					arg.push(c);
// 				}
// 				_ => {
// 					arg.push(c);
// 				}
// 			}
// 		} else {
// 			match c {
// 				' ' => {
// 					if arg.len() > 0 {
// 						args.push_back(arg);
// 						arg = String::new();
// 					}
// 				}
// 				'\\' => {
// 					isescaped = true;
// 				}
// 				'$' => {
// 					iseval = true;
// 					brackets = 0;
// 				}
// 				_ => {
// 					arg.push(c);
// 				}
// 			}
// 		}
// 	}
// 	if arg.len() > 0 {
// 		if iseval {
// 			if brackets <= 0 {
// 				args.push_back(shell.eval(&arg)?);
// 			}
// 		} else {
// 			args.push_back(arg);
// 		}
// 	}
// 	args
// }