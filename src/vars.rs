
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::io::Error;
use std::clone::Clone;

use super::shell;
use super::args;

type Func = fn(&mut shell::Shell, args: &args::Args) -> Result<String, Error>;
type Value = String;
type Namespace = Rc<Vars>;

pub enum Var {
	Value(Value),
	Func(Func),
	Namespace(Namespace)
}

impl Var {
	pub fn copy(&self) -> Var {
		match self {
			Var::Value(v) => Var::Value(v.clone()),
			Var::Func(f) => Var::Func(f.clone()),
			Var::Namespace(n) => Var::Namespace(n.clone()),
		}
	}
}

impl std::fmt::Debug for Var {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		formatter.write_str(
			match self {
				Var::Value(v) => v,
				Var::Func(_) => "function",
				Var::Namespace(_) => "namespace",
			}
		)
	}
}

pub struct Vars {
	data: HashMap<u64, Rc<Var>>
}

impl Vars {
	pub fn new() -> Vars {
		let hashmap = HashMap::new();
		Vars {
			data: hashmap
		}
	}
	pub fn importnamespace<T: Hash>(&mut self, key: T) -> Result<(), Error> {
		// let namespaceopt = self.get(key)?;
		let namespaceopt = &**self.data.get(&self.hash(key)).unwrap();
		let namespace;
		match namespaceopt {
			Var::Value(_) => { return Err(Error::new(std::io::ErrorKind::InvalidInput, "Got Value, expected Namespace")) }
			Var::Func(_) => { return Err(Error::new(std::io::ErrorKind::InvalidInput, "Got Value, expected Function")) }
			Var::Namespace(n) => { namespace = n; }
		}
		println!("hash for readline: {:?}", self.hash("readline"));
		for (key, var) in namespace.data.clone() {
			println!("{:?}: {:?}", key, var);
			self.data.insert(key, var);
		}
		println!("out for readline: {:?}", self.get("readline")?);
		Ok(())
	}
	pub fn hash<T: Hash>(&self, key: T) -> u64 {
		let mut hasher = DefaultHasher::new();
		hasher.update();
		key.hash(&mut hasher);
		hasher.update(msg);
		format!("{:x}", hasher.())
		hasher.finish()
	}
	pub fn get<T: Hash>(&self, key: T) -> Result<&Rc<Var>, Error> {
		let out = self.data.get(&self.hash(key));
		if out.is_none() {
			Err(Error::new(std::io::ErrorKind::NotFound, "Variable not found"))
		} else {
			Ok(out.unwrap())
		}
	}
	pub fn set<T: Hash>(&mut self, key: T, value: Var) {
		self.data.insert(self.hash(key), Rc::new(value));
	}
}