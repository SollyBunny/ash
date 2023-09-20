
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Error;

pub enum Type {
	Unknown,
	Value,
	Func
}


type Func = fn(&Vars) -> Result<&str, Error>;
type Value = String;

pub enum Var {
	Value(Value),
	Func(Func)
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