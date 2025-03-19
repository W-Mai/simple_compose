use mutheors::*;

macro_rules! degrees {
    ($pitch_class:expr => $($degree:expr)*) => {
        [$($degree),*].map(|degree| $pitch_class.common_chord(degree, 4))
    };
}

const BEAT: u8 = 3;
const BEAT_TYPE: DurationBase = DurationBase::Quarter;

///
/// ```plaintext
/// PitchClass ----------------> Chord -------------> Note
///              common_chord            breakdown
/// ```
fn main() {
    // Declares the modulation mode
    let pitch_classes = [
        PitchClass::C,
        PitchClass::F,
        PitchClass::D,
        PitchClass::G,
        PitchClass::C,
    ];

    // Declares a score with 2 tracks
    // and a tempo of Vivace
    // and a time signature of 3/4
    let mut score = Score::<2>::new()
        .with_tempo(Tempo::Vivace)
        .with_time_signature(BEAT, BEAT_TYPE);

    // Get the duration generator from the score. Currently, it is set to 3/4.
    // It means each beat is a quarter note.
    let dg = score.duration_generator();

    // Generate the measures for each pitch class
    for pitch_class in pitch_classes {
        // Generate the chord for each pitch class
        // Current it will generate a chord sequence of
        // 1 1 4 5 1 4 1 degrees
        let chords = degrees!(pitch_class
            => 1 1 4 5 1 4 1
        );

        // Generate the measures for each chord
        (0..chords.len()).for_each(|i| {
            score.new_measures(|m| {
                // Set the first track to play the chord
                m[0].chord(chords[i].clone());
                // Set the second track to play the notes of the chord randomly
                m[1] = duration_utils::generate_one_measure(&dg, chords[i].clone(), BEAT);
            })
        });
    }

    // Play the score
    score.play("Simple Compose").unwrap()
}
