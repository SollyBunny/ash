
use std::collections::HashMap;
use std::string::ToString;
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
	data: HashMap<String, Rc<Var>>
}

impl Vars {
	pub fn new() -> Vars {
		let hashmap = HashMap::new();
		Vars {
			data: hashmap
		}
	}
	pub fn importnamespace<T: ToString>(&mut self, key: T) -> Result<(), Error> {
		// let namespaceopt = self.get(key)?;
		let namespaceopt = &**self.data.get(&key.to_string()).unwrap();
		let namespace;
		match namespaceopt {
			Var::Value(_) => { return Err(Error::new(std::io::ErrorKind::InvalidInput, "Got Value, expected Namespace")) }
			Var::Func(_) => { return Err(Error::new(std::io::ErrorKind::InvalidInput, "Got Value, expected Function")) }
			Var::Namespace(n) => { namespace = n; }
		}
		for (key, var) in namespace.data.clone() {
			println!("{:?}: {:?}", key, var);
			self.data.insert(key, var);
		}
		Ok(())
	}
	pub fn get<T: ToString + std::fmt::Debug>(&self, key: T) -> Result<&Rc<Var>, Error> {
		let out = self.data.get(&key.to_string());
		if out.is_none() {
			Err(Error::new(std::io::ErrorKind::NotFound, format!("Variable {:?} not found", key)))
		} else {
			Ok(out.unwrap())
		}
	}
	pub fn set<T: ToString>(&mut self, key: T, value: Var) {
		self.data.insert(key.to_string(), Rc::new(value));
	}
}