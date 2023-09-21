
pub struct Args {
	pub v: Vec<String>
}

impl Args {
	pub fn new() -> Args {
		let mut vec: Vec<String> = Vec::new();
		vec.push("".to_string());
		Args {
			v: vec
		}
	}
}