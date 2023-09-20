
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Error;

use super::shell;
use super::args;

type Func = fn(&mut shell::Shell, args: &args::Args) -> Result<String, Error>;
type Value = String;
type Namespace = Vars;

pub enum Var {
	Value(Value),
	Func(Func),
	Namespace(Vars)
}

impl Var {
	pub fn copy(&self) -> Var {
		match self {
			Var::Value(v) => Var::Value(v.copy()),
			Var::Func(f) => Var::Func(f.copy()),
			Var::Namespace(n) => Var::Namespace(_n),
		}
	}
}

pub struct Vars {
	data: HashMap<u64, Var>
}

impl Vars {
	pub fn new() -> Vars {
		Vars {
			data: HashMap::new()
		}
	}
	pub fn add(&mut self, vars: &Vars) {
		for (key, value) in &vars.data {
			self.data.insert(*key, value.copy());
		}
	}
	fn hash(&self, key: &String) -> u64 {
		let mut hasher = DefaultHasher::new();
		key.hash(&mut hasher);
		hasher.finish()
	}
	pub fn get(&self, key: &String) -> Option<&Var> {
		self.data.get(&self.hash(key))
	}
	pub fn set(&mut self, key: &String, value: Var) {
		self.data.insert(self.hash(key), value);
	}
}