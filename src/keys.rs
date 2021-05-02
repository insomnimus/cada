use cada::sound::Note;
use cada::wave::WaveForm;
use std::sync::Arc;
use std::collections::HashMap;
use rdev::Key;

pub fn generate_mappings() -> HashMap<Key, Arc<Note>> {
	let nn= |fq: f32| -> Arc<Note> {
		Arc::new(Note::new(WaveForm::sample(fq, 44100)))
	};
	let mut keys: HashMap<Key, Arc<Note>>= HashMap::new();
	{// just for grouping together
	keys.insert(Key::KeyQ, nn(207.65);
keys.insert(Key::KeyA, nn(220.00);
keys.insert(Key::KeyW, nn(233.08);
keys.insert(Key::KeyS, nn(246.94);
keys.insert(Key::KeyD, nn(261.63);
keys.insert(Key::KeyE, nn(277.18);
keys.insert(Key::KeyF, nn(293.66);
keys.insert(Key::KeyT, nn(311.13);
keys.insert(Key::KeyG, nn(329.63);
keys.insert(Key::KeyH, nn(349.23);
keys.insert(Key::KeyY, nn(369.99);
keys.insert(Key::KeyJ, nn(392.00);
keys.insert(Key::KeyI, nn(415.30);
keys.insert(Key::KeyK, nn(440.00);
keys.insert(Key::KeyO, nn(466.16);
keys.insert(Key::KeyL, nn(493.88);
keys.insert(Key::SemiColon, nn(523.25);
keys.insert(Key::KeyP, nn(554.37
	}
	keys
}