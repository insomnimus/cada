use std::sync::Mutex;

pub struct Note {
    sample: Vec<f32>,
    paused: Mutex<bool>,
    clock: Mutex<usize>,
}

impl Note {
    pub fn new(sample: Vec<f32>) -> Self {
        Self {
            sample,
            clock: Mutex::new(0usize),
            paused: Mutex::new(true),
        }
    }

    pub fn next(&self) -> f32 {
        let paused = self.paused.lock().unwrap();
        if *paused {
            return 0f32;
        }
        let mut n = self.clock.lock().unwrap();

        if *n >= self.sample.len() - 1 {
            *n = 0;
        } else {
            *n += 1;
        }
        self.sample[*n]
    }

    pub fn play(&self) {
        *self.paused.lock().unwrap() = false;
    }

    pub fn mute(&self) {
        *self.paused.lock().unwrap() = true;
    }
}
