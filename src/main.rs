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
    let chords = degrees!(PitchClass::F
        => 1 1 4 5 1 4 1
    );

    let mut score = Score::<2>::new()
        .with_tempo(Tempo::Allegro)
        .with_time_signature(BEAT, BEAT_TYPE);
    let dg = score.duration_generator();

    (0..chords.len()).for_each(|i| {
        score.new_measures(|m| {
            m[0].chord(chords[i].clone());
            m[1] = duration_utils::generate_one_measure(&dg, chords[i].clone(), BEAT);
        })
    });

    let mut midi_player = MidiPlayer::new("Simple Compose");
    midi_player.play_score(score).unwrap();

    midi_player.close();
}
