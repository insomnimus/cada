use crate::wave::WaveForm;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct Note {
	sample: Vec<f32>,
	fq: f32,
	sr: usize,
	paused: AtomicBool,
	clock: AtomicUsize,
}

impl Note {
	pub fn new(sample: Vec<f32>, fq: f32, sr: usize) -> Self {
		Self {
			sr,
			fq,
			sample,
			clock: AtomicUsize::new(0usize),
			paused: AtomicBool::new(true),
		}
	}

	pub fn next(&self) -> f32 {
		if self.paused.fetch_or(false, Ordering::Relaxed) {
			return 0f32;
		}
		unsafe {
			let n = self.clock.as_mut_ptr();

			if *n >= self.sample.len() - 1 {
				*n = 0;
			} else {
				*n += 1;
			}
			self.sample[*n]
		}
	}

	pub fn play(&self) {
		self.paused.store(false, Ordering::Relaxed);
	}

	pub fn mute(&self) {
		self.clock.store(0usize, Ordering::Relaxed);
		self.paused.store(true, Ordering::Relaxed);
	}

	pub fn add(mut self, w: WaveForm) -> Self {
		let sample = WaveForm::sample(w, self.fq, self.sr);
		self.sample = self
			.sample
			.into_iter()
			.zip(sample.into_iter())
			.map(|(x, y)| (x + y) / 2f32)
			.collect();
		self
	}
}
