pub struct Task {
	name: String,
}

impl Task {
	pub fn new(name: String) -> Task {
		Task { name }
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}
