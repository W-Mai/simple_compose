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
    let deg = degrees!(1 1 4 5 1 4);
    let chords = deg.map(|degree| pitch_class.common_chord(degree));

    let mut rng = thread_rng();

    let mut midi_player = MidiPlayer::new("Simple Compose");
    let ports = midi_player.list_ports();
    let need_play = ports.len() > 0;
    if need_play {
        midi_player.select_port(0).unwrap();
        midi_player.connect("Simple Compose Port 0").unwrap();
    }

    for chord in chords {
        let notes = chord
            .components()
            .iter()
            .map(|note| Note {
                pitch_class,
                octave: note.octave as u8,
                duration: 0.0,
                velocity: 0.0,
            })
            .collect::<Vec<_>>();
        let durations = duration_utils::generate_one_measure(4);
        for duration in durations {
            let duration_value = duration.clone().into();
            let note = notes.choose(&mut rng).unwrap().clone();
            let note = note.with_duration(duration_value);
            let s = format!("{}[{}]", note, duration);
            print!("{} ", s);

            if need_play {
                fn to_midi(note: &Note) -> u8 {
                    note.pitch_class as u8 + (note.octave + 1) * 12 - 1
                }

                let notes = note
                    .pitch_class
                    .common_chord(1)
                    .components()
                    .iter()
                    .map(|x| x.midi_number().unwrap())
                    .collect::<Vec<_>>();

                midi_player.play_notes(&notes);
                sleep(Duration::from_millis((duration_value * 80.0 * 8.0) as u64));
                midi_player.stop_notes(&notes);
            }
        }
        println!("|");
    }

    midi_player.close();
}
