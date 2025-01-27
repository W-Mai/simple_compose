#[cfg(test)]
mod tests {
    use simple_compose::*;

    #[test]
    fn test_modulation() {
        assert_eq!(Tuning::C.modulation(4), Tuning::E);
        assert_eq!(Tuning::C.modulation(-3), Tuning::A);
        assert_eq!(Tuning::C.modulation(12), Tuning::C);
        assert_eq!(Tuning::C.modulation(-12), Tuning::C);
        assert_eq!(Tuning::C.modulation(0), Tuning::C);
        assert_eq!(Tuning::None.modulation(1), Tuning::None);
    }

    #[test]
    fn test_breakdown() {
        let tuning = Tuning::C;
        let chord = tuning.common_chord(1);
        assert_eq!(chord.tonality, Tonality::Major);
        let notes = chord.breakdown(4);
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].tuning, Tuning::C);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].tuning, Tuning::E);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].tuning, Tuning::G);
        assert_eq!(notes[2].octave, 4);

        let chord = tuning.common_chord(2);
        assert_eq!(chord.tonality, Tonality::Minor);
        let notes = chord.breakdown(4);
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].tuning, Tuning::D);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].tuning, Tuning::F);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].tuning, Tuning::A);
        assert_eq!(notes[2].octave, 4);

        let chord = tuning.common_chord(6);
        assert_eq!(chord.tonality, Tonality::Minor);
    }

    #[test]
    fn test_breakdown_2() {
        let tuning = Tuning::CSharpOrDFlat;
        let chord = tuning.common_chord(1);
        let notes = chord.breakdown(4);

        assert_eq!(chord, Chord { tuning: Tuning::CSharpOrDFlat, tonality: Tonality::Major });
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].tuning, Tuning::CSharpOrDFlat);
        assert_eq!(notes[0].octave, 4);
        assert_eq!(notes[1].tuning, Tuning::F);
        assert_eq!(notes[1].octave, 4);
        assert_eq!(notes[2].tuning, Tuning::GSharpOrAFlat);
        assert_eq!(notes[2].octave, 4);
    }

    #[test]
    fn test_tuning_1() {
        let tuning = Tuning::CSharpOrDFlat;
        assert_eq!(tuning.next_basic_degree(0), Tuning::CSharpOrDFlat);
        assert_eq!(tuning.next_basic_degree(1), Tuning::DSharpOrEFlat);
        assert_eq!(tuning.next_basic_degree(2), Tuning::F);
        assert_eq!(tuning.next_basic_degree(3), Tuning::FSharpOrGFlat);
        assert_eq!(tuning.next_basic_degree(4), Tuning::GSharpOrAFlat);
        assert_eq!(tuning.next_basic_degree(5), Tuning::ASharpOrBFlat);
        assert_eq!(tuning.next_basic_degree(6), Tuning::C);
        assert_eq!(tuning.next_basic_degree(7), Tuning::CSharpOrDFlat);
    }
}
