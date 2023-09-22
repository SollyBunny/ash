
use std::io::Error;

use super::shell;
use super::vars;
use super::args;

mod builtins;

pub fn add(shell: &mut shell::Shell) -> Result<(), Error> {
	builtins::add(shell)
}