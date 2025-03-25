use mutheors::duration_utils::DurationProgress;
use mutheors::*;

macro_rules! degrees {
    ($scale:expr => $($degree:expr)*) => {
        [$($degree),*].map(|degree| $scale.degree_chord(degree).unwrap())
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
    let scale_types = [ScaleType::Major, ScaleType::NaturalMinor];

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
        let scale = Tuning::new(pitch_class, 4).scale(scale_types[pitch_class as usize % 2]);
        let chords = degrees!(scale
            => 1 1 4 5 1 4 1
        );

        // Generate the measures for each chord
        (0..chords.len()).for_each(|i| {
            score.new_measures(|m| {
                // Set the first track to play the chord
                m[0].chord(chords[i].clone());
                // Set the second track to play the notes of the chord randomly
                m[1] = duration_utils::generate_one_measure(
                    &dg,
                    chords[i].clone(),
                    BEAT,
                    DurationProgress::Fixed(vec![1.0, 0.5, 0.5, 1.0]),
                );
            })
        });
    }

    // Play the score
    score.play("Simple Compose").unwrap()
}
