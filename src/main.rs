use termios::*;
use std::io;

fn main() {
	let mut termios = Termios::from_fd(io::stdin().as_raw_fd()).unwrap();
	println!("Hello World!");
}