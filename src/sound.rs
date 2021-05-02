use std::cell::RefCell;

pub struct Note {
	sample: Vec<f32>,
	paused: RefCell<bool>,
	clock: RefCell<u32>,
}

impl Note {
	pub fn new(samples: Vec<f32>) -> Self {
		Self {
			samples,
			clock: RefCell::new(0u32),
			paused: RefCell::new(true),
		}
	}

	pub fn next(&mut self) -> f32 {
		if self.paused.try_borrow().unwrap_or(true) {
			return 0;
		}
		let mut n = match self.clock.try_borrow_mut() {
			Ok(val) => val,
			_ => return 0,
		};
		if *n >= self.sample.len() {
			*n = 0;
		} else {
			*n += 1;
		}
		self.samples[*n]
	}

	pub fn play(&self) {
		self.paused.replace(false);
	}

	pub fn mute(&self) {
		self.paused.replace(true);
	}
}
