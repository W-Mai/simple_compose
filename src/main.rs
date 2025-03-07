use rand::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use simple_compose::*;

macro_rules! degrees {
    ($($degree:expr)*) => {
        [$($degree),*]
    };
}

///
/// ```plaintext
/// PitchClass ----------------> Chord -------------> Note
///         common_chord()          breakdown()
/// ```
fn main() {
    let pitch_class = PitchClass::C;
    let deg = degrees!(1 5 6 3 4 1 4 5);
    let chords = deg.map(|degree| pitch_class.common_chord(degree, 3));

    let mut rng = thread_rng();

    let mut midi_player = MidiPlayer::new("Simple Compose");
    let ports = midi_player.list_ports();
    let mut channels = None;
    let need_play = ports.len() > 0;
    if need_play {
        midi_player.select_port(0).unwrap();
        channels.replace(midi_player.connect("Simple Compose Port 0").unwrap());
    }

    for chord in chords.clone() {
        let chord_notes = chord
            .components()
            .iter()
            .map(|note| Note {
                pitch_class: note.class,
                octave: note.octave,
                duration: 0.0,
                velocity: 0.0,
            })
            .collect::<Vec<_>>();

        let chord_notes_midi = chord
            .components()
            .iter()
            .map(|x| x.midi_number().unwrap() + 12 * 1)
            .collect::<Vec<_>>();
        if let Some(channels) = channels.as_mut() {
            channels[2].play_notes(&chord_notes_midi);
        }
        let durations = duration_utils::generate_one_measure(1);
        for duration in durations {
            let duration_value = duration.clone().into();
            let note = chord_notes.choose(&mut rng).unwrap().clone();
            let note = note.with_duration(duration_value);
            let s = format!("{}[{}]", note, duration);
            print!("{} ", s);

            if let Some(channels) = channels.as_mut() {
                let tuning_midi = [Tuning::new(note.pitch_class, note.octave)
                    .midi_number()
                    .unwrap()];
                channels[1].play_notes(&tuning_midi);
                sleep(Duration::from_millis((duration_value * 80.0 * 32.0) as u64));
                channels[1].stop_notes(&tuning_midi);
            }
        }
        if let Some(channels) = channels.as_mut() {
            channels[2].stop_notes(&chord_notes_midi);
        }
        println!("|");
    }

    midi_player.close();
}
