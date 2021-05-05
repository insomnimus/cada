pub enum WaveForm {
	Square,
	Saw,
	Triangle,
	Sine,
}

impl WaveForm {
	pub fn sample(self, fq: f32, sr: usize) -> Vec<f32> {
		match self {
			Self::Sine => Self::sine(fq, sr),
			Self::Triangle => Self::triangle(fq, sr),
			Self::Square => Self::square(fq, sr),
			// WaveForm::Sine=> Self::sine(fq, sr),
			Self::Saw => Self::saw(fq, sr),
		}
	}

	fn triangle(fq: f32, sr: usize) -> Vec<f32> {
		let n = sr as f32 / fq;
		let n = n as i32;
		let mut up = true;
		let step: f32 = (fq * 4.0) / (sr as f32);
		let mut buff: Vec<f32> = vec![];

		let mut y = 0f32;
		for _ in 0..n {
			if up {
				y += step;
				if y >= 1.0 {
					up = false;
				}
			} else {
				y -= step;
				if y <= -1.0 {
					up = true;
				}
			}
			buff.push(y);
		}
		buff
	}

	fn square(fq: f32, sr: usize) -> Vec<f32> {
		let n = sr as f32 / fq;
		let n = n as i32;
		let mut up = true;
		let step: f32 = (fq * 4.0) / (sr as f32);
		let mut buff: Vec<f32> = vec![];

		let mut y = 0f32;
		for _ in 0..n {
			if up {
				y += step;
				if y >= 1.0 {
					up = false;
				}
			} else {
				y -= step;
				if y <= -1.0 {
					up = true;
				}
			}
			buff.push(if y < 0.0 { -1.0 } else { 1.0 });
		}
		buff
	}

	fn saw(fq: f32, sr: usize) -> Vec<f32> {
		let n = sr as f32 / fq;
		let n = n as i32;
		let step: f32 = (fq * 4.0) / (sr as f32);
		let mut buff: Vec<f32> = vec![];

		let mut y = 0f32;
		for _ in 0..n {
			y += step;
			if y >= 1.0 {
				y = -1.0;
			}
			buff.push(y);
		}
		buff
	}

	fn sine(fq: f32, sr: usize) -> Vec<f32> {
		let n = sr as f32 / fq;
		let n = n as i32;
		let mut clock = 0f32;
		let mut sample = vec![];
		for _ in 0..n {
			clock = (clock + 1.0) % sr as f32;
			sample.push((clock * fq * 2.0 * std::f32::consts::PI / sr as f32).sin());
		}
		sample
	}
}
