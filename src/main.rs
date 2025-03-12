use rand::prelude::*;
use rand::rng;

use mutheors::*;

macro_rules! degrees {
    ($($degree:expr)*) => {
        [$($degree),*]
    };
}

///
/// ```plaintext
/// PitchClass ----------------> Chord -------------> Note
///              common_chord            breakdown
/// ```
fn main() {
    let pitch_class = PitchClass::G;
    let deg = degrees!(1 1 4 5 1 4 1);
    let chords = deg.map(|degree| pitch_class.common_chord(degree, 3));

    let mut score = Score::<2>::new().with_tempo(Tempo::Allegro);
    let mut rng = rng();

    (0..deg.len()).for_each(|i| {
        score.new_measures(|m| {
            m[0].chord(chords[i].clone());

            let chord_notes = chords[i].components();
            let durations = duration_utils::generate_one_measure(4);
            let note_iter = durations
                .iter()
                .map(|duration| {
                    let tuning = chord_notes.choose(&mut rng).unwrap().clone();
                    Note::new(tuning.add_interval(&Interval::from_semitones(12).unwrap()))
                        .with_duration(duration.clone())
                })
                .collect();

            m[1].note(note_iter);
        })
    });

    let mut midi_player = MidiPlayer::new("Simple Compose");
    midi_player.play_score(score).unwrap();

    midi_player.close();
}
