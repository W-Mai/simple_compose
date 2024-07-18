use std::fmt::Display;

///
/// ```plaintext
/// Tuning ----------------> Chord -------------> Note
///         common_chord()          breakdown()
/// ```
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
#[derive(PartialEq)]
enum Tuning {
    None = 0,
    C = 1,
    D = 2,
    E = 3,
    F = 4,
    G = 5,
    A = 6,
    B = 7,
}

impl From<u8> for Tuning {
    fn from(value: u8) -> Self {
        match value {
            1 => Tuning::C,
            2 => Tuning::D,
            3 => Tuning::E,
            4 => Tuning::F,
            5 => Tuning::G,
            6 => Tuning::A,
            7 => Tuning::B,
            _ => panic!("Invalid value"),
        }
    }
}

impl Tuning {
    pub fn modulation(&self, degree: i8) -> Tuning {
        match self {
            Tuning::None => { Tuning::None }
            _ => Tuning::from(((*self as i8 - 1 + 7 + degree) % 7 + 1) as u8)
        }
    }

    pub fn common_chord(&self, degree: u8) -> Chord {
        assert!(degree > 0 && degree < 8, "Degree must be in [1, 6]");
        let new_tuning = self.modulation((degree - 1) as i8);
        let tonality = match degree {
            1 | 4 | 5 => Tonality::Major,
            2 | 3 | 6 => Tonality::Minor,
            _ => panic!("Invalid degree"),
        };

        Chord { tuning: new_tuning, tonality }
    }
}

impl Display for Tuning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Tuning::C => "C",
            Tuning::D => "D",
            Tuning::E => "E",
            Tuning::F => "F",
            Tuning::G => "G",
            Tuning::A => "A",
            Tuning::B => "B",
            Tuning::None => " "
        }.to_string();
        write!(f, "{}", str)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
#[derive(PartialEq)]
enum Tonality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Perfect,
}

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq)]
struct Chord {
    tuning: Tuning,
    tonality: Tonality,
}

impl Default for Chord {
    fn default() -> Self {
        Chord {
            tuning: Tuning::C,
            tonality: Tonality::Major,
        }
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tuning_str = self.tuning.to_string();
        let tonality_str = match self.tonality {
            Tonality::Major => "",
            Tonality::Minor => "m",
            _ => "?"
        };

        write!(f, "{}", format!("{}{}", tuning_str, tonality_str))
    }
}

impl Chord {
    pub fn breakdown(&self, octave: u8) -> Vec<Note> {
        fn octave_eval(octave: u8, base_degree: Tuning, delta_degree: i8) -> u8 {
            let new_degree = base_degree as i8 + delta_degree;
            (octave as i8 + new_degree / 7 + (new_degree % 7 != 0) as i8 - 1) as u8
        }

        match self.tonality {
            Tonality::Major => {
                vec![
                    Note { chord: self.tuning, octave, duration: 0.5, velocity: 0.5 },
                    Note { chord: self.tuning.modulation(2), octave: octave_eval(octave, self.tuning, 2), duration: 0.5, velocity: 0.5 },
                    Note { chord: self.tuning.modulation(4), octave: octave_eval(octave, self.tuning, 4), duration: 0.5, velocity: 0.5 },
                ]
            }
            Tonality::Minor => {
                vec![
                    Note { chord: self.tuning, octave, duration: 0.5, velocity: 0.5 },
                    Note { chord: self.tuning.modulation(2), octave: octave_eval(octave, self.tuning, 2), duration: 0.5, velocity: 0.5 },
                    Note { chord: self.tuning.modulation(4), octave: octave_eval(octave, self.tuning, 4), duration: 0.5, velocity: 0.5 },
                ]
            }
            _ => vec![]
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Note {
    chord: Tuning,
    octave: u8,
    duration: f32,
    velocity: f32,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chord_str = self.chord.to_string();
        let octave_str = self.octave.to_string();

        write!(f, "{}{}", chord_str, octave_str)
    }
}

fn main() {
    let chord = Tuning::C;

    for degree in 1..=6 {
        println!("{}", chord.common_chord(degree));
    }

    let notes = Tuning::C.common_chord(6).breakdown(4);

    for note in notes {
        println!("{}", note);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulation() {
        assert_eq!(Tuning::C.modulation(2), Tuning::E);
        assert_eq!(Tuning::C.modulation(-2), Tuning::A);
        assert_eq!(Tuning::C.modulation(7), Tuning::C);
        assert_eq!(Tuning::C.modulation(-7), Tuning::C);
        assert_eq!(Tuning::C.modulation(0), Tuning::C);
        assert_eq!(Tuning::None.modulation(1), Tuning::None);
    }
}
