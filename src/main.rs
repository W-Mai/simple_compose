use rand::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use simple_compose::*;

macro_rules! degrees {
    ($($degree:expr)*) => {
        [$($degree),*]
    };
}

struct Measure {
    chord: Chord,
    chord_notes: Vec<Tuning>,
    rhythm_notes: Vec<Note>,
}

///
/// ```plaintext
/// PitchClass ----------------> Chord -------------> Note
///         common_chord()          breakdown()
/// ```
fn main() {
    let pitch_class = PitchClass::DSharpOrEFlat;
    let deg = degrees!(1 1 4 5 1 4 1);
    let chords = deg.map(|degree| pitch_class.common_chord(degree, 3));

    let mut rng = thread_rng();

    let measures = chords.map(|chord| {
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

        let chord_tunings = chord.components();

        let durations = duration_utils::generate_one_measure(1);
        let note_iter = durations.iter().map(|duration| {
            let duration_value = duration.clone().into();
            let note = chord_notes.choose(&mut rng).unwrap().clone();
            note.with_duration(duration_value)
        });

        Measure {
            chord: chord.clone(),
            chord_notes: chord_tunings,
            rhythm_notes: note_iter.collect(),
        }
    });

    let mut midi_player = MidiPlayer::new("Simple Compose");
    let ports = midi_player.list_ports();
    let need_play = ports.len() > 0;
    if need_play {
        midi_player.select_port(0).unwrap();
        let channels = &mut midi_player.connect("Simple Compose Port 0").unwrap();
        let mut ch_chords = channels[0].borrow_mut();
        let mut ch_rhythm = channels[1].borrow_mut();

        measures.iter().for_each(
            |Measure {
                 chord,
                 chord_notes,
                 rhythm_notes,
             }| {
                print!("{} {:?} | ", chord_notes[0].class, chord.quality());
                let chord_notes_midi = chord_notes
                    .iter()
                    .map(|x| x.midi_number().unwrap() - 12 * 1)
                    .collect::<Vec<u8>>();
                ch_chords.play_notes(&chord_notes_midi);
                rhythm_notes.iter().for_each(|note| {
                    let s = format!("{}[{}]", note, note.duration);
                    print!("{} ", s);
                    let tuning_midi = [Tuning::new(note.pitch_class, note.octave)
                        .midi_number()
                        .unwrap()];
                    ch_rhythm.play_notes(&tuning_midi);
                    sleep(Duration::from_millis((note.duration * 80.0 * 32.0) as u64));
                    ch_rhythm.stop_notes(&tuning_midi);
                });
                ch_chords.stop_notes(&chord_notes_midi);

                println!("|");
            },
        );
    } else {
        measures.iter().for_each(
            |Measure {
                 chord,
                 chord_notes,
                 rhythm_notes,
             }| {
                print!("{} {:?} | ", chord_notes[0].class, chord.quality());
                rhythm_notes.iter().for_each(|note| {
                    let s = format!("{}[{}]", note, note.duration);
                    print!("{} ", s);
                });
                println!("|");
            },
        );
    }
    midi_player.close();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_with_midi_player() {
        let mut score = Score::<2>::new().with_tempo(140.0);
        score.new_measures(|m| {
            m[0].rest();
            m[1].note(vec![
                Note::new(PitchClass::C, 4).with_duration(DurationBase::Quarter.in_quarters()),
                Note::new(PitchClass::E, 4).with_duration(DurationBase::Eighth.in_quarters()),
                Note::new(PitchClass::G, 4).with_duration(DurationBase::Eighth.in_quarters()),
                Note::new(PitchClass::B, 4).with_duration(DurationBase::Eighth.in_quarters()),
                Note::new(PitchClass::D, 5).with_duration(DurationBase::Eighth.in_quarters()),
            ]);
        });
        score.new_measures(|m| {
            m[0].chord(Chord::triad(Tuning::new(PitchClass::G, 4), ChordQuality::Major).unwrap());
            m[1].chord(Chord::triad(Tuning::new(PitchClass::A, 4), ChordQuality::Major).unwrap());
        });
        score.new_measures(|m| {
            m[0].note(vec![
                Note::new(PitchClass::C, 4).with_duration(DurationBase::Quarter.in_quarters()),
                Note::new(PitchClass::E, 4).with_duration(DurationBase::Quarter.in_quarters()),
                Note::new(PitchClass::B, 4).with_duration(DurationBase::Eighth.in_quarters()),
                Note::new(PitchClass::G, 4).with_duration(DurationBase::Eighth.in_quarters()),
                Note::new(PitchClass::E, 4).with_duration(DurationBase::Eighth.in_quarters()),
                Note::new(PitchClass::C, 5).with_duration(DurationBase::Eighth.in_quarters()),
            ]);
            m[1].rest();
        });

        let mut midi_player = MidiPlayer::new("Simple Compose");
        midi_player.play_score(score).unwrap();
    }
}
