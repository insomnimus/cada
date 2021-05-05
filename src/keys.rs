use crate::sound::Note;
use crate::wave::WaveForm;
use rdev::Key;
use std::collections::HashMap;
use std::sync::Arc;

pub fn generate_mappings(sr: f32) -> HashMap<char, Arc<Note>> {
	let nn = |fq: f32| -> Arc<Note> {
		Arc::new(
			Note::new(
				WaveForm::sample(WaveForm::Sine, fq, sr as usize),
				fq,
				sr as usize,
			)
			.add(WaveForm::Saw),
		)
	};

	let mut keys = HashMap::new();
	{
		// just to group
		keys.insert('q', nn(207.65));
		keys.insert('a', nn(220.00));
		keys.insert('w', nn(233.08));
		keys.insert('s', nn(246.94));
		keys.insert('d', nn(261.63));
		keys.insert('e', nn(277.18));
		keys.insert('f', nn(293.66));
		keys.insert('t', nn(311.13));
		keys.insert('g', nn(329.63));
		keys.insert('h', nn(349.23));
		keys.insert('y', nn(369.99));
		keys.insert('j', nn(392.00));
		keys.insert('i', nn(415.30));
		keys.insert('k', nn(440.00));
		keys.insert('o', nn(466.16));
		keys.insert('l', nn(493.88));
		keys.insert(';', nn(523.25));
		keys.insert('p', nn(554.37));
	}
	keys
}

pub fn map_key(k: Key) -> Option<char> {
	match k {
		Key::KeyQ => Some('q'),
		Key::KeyA => Some('a'),
		Key::KeyW => Some('w'),
		Key::KeyS => Some('s'),
		Key::KeyD => Some('d'),
		Key::KeyE => Some('e'),
		Key::KeyF => Some('f'),
		Key::KeyT => Some('t'),
		Key::KeyG => Some('g'),
		Key::KeyH => Some('h'),
		Key::KeyY => Some('y'),
		Key::KeyJ => Some('j'),
		Key::KeyI => Some('i'),
		Key::KeyK => Some('k'),
		Key::KeyO => Some('o'),
		Key::KeyL => Some('l'),
		Key::SemiColon => Some(';'),
		Key::KeyP => Some('p'),
		_ => None,
	}
}
