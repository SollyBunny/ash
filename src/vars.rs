
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

impl std::fmt::Display for Var {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		let string: String = match self {
			Var::Value(v) => format!("= {}", v),
			Var::Func(_f) => format!("<function>"),
			Var::Namespace(_n) => format!("<namespace>"),
		};
		formatter.write_str(string.as_str())
	}
}

pub struct Vars {
	pub data: HashMap<String, Rc<Var>>
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