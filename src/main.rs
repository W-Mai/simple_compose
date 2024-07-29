use rand::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use simple_compose::*;

macro_rules! degrees {
    ($($degree:expr)*) => {
        [$($degree),*]
    };
}

// 按照 1/32 1/16 1/8 1/4 1/2 1 2 4 的时值随机生成一个时值序列，并保证生成的时值序列长度为 4
fn generate_duration() -> Vec<f32> {
    let mut durations = vec![];
    let mut rng = thread_rng();
    let mut duration_sum = 0.0;
    while duration_sum < 4.0 {
        let duration = match rng.gen_range(3..=5) {
            0 => 1.0 / 32.0,
            1 => 1.0 / 16.0,
            2 => 1.0 / 8.0,
            3 => 1.0 / 4.0,
            4 => 1.0 / 2.0,
            5 => 1.0,
            6 => 2.0,
            7 => 4.0,
            _ => unreachable!(),
        };
        if duration_sum + duration > 4.0 {
            break;
        }
        duration_sum += duration;
        durations.push(duration);
    }

    let remainder = duration_sum - 4.0;
    if remainder > 0.0 {
        durations.push(remainder);
    }
    durations
}

///
/// ```plaintext
/// Tuning ----------------> Chord -------------> Note
///         common_chord()          breakdown()
/// ```
fn main() {
    let tuning = Tuning::C;
    let deg = degrees!(1 1 4 5 1 4);
    let chords = deg.map(|degree| tuning.common_chord(degree));

    let mut rng = thread_rng();

    let mut midi_player = MidiPlayer::new("Simple Compose");
    let ports = midi_player.list_ports();
    let need_play = ports.len() > 0;
    if need_play {
        midi_player.select_port(0).unwrap();
        midi_player.connect("Simple Compose Port 0").unwrap();
    }

    for chord in chords {
        let notes = chord.breakdown(4);
        let durations = generate_duration();
        for duration in durations {
            let note = notes.choose(&mut rng).unwrap().clone();
            let note = note.with_duration(duration);
            print!("{}[{}] ", note, duration);

            if need_play {
                fn to_midi(note: &Note) -> u8 {
                    note.tuning as u8 + (note.octave + 1) * 12 - 1
                }

                let notes = note
                    .tuning
                    .common_chord(1)
                    .breakdown(note.octave)
                    .iter()
                    .map(to_midi)
                    .collect::<Vec<_>>();

                midi_player.play_notes(&notes);
                sleep(Duration::from_millis((duration * 80.0 * 8.0) as u64));
                midi_player.stop_notes(&notes);
            }
        }
        println!("|");
    }

    midi_player.close();
}
