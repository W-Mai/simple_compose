use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
#[derive(PartialEq)]
enum Tuning {
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
        let value = ((*self as i8 - 1 + 7 + degree) % 7 + 1) as u8;
        Tuning::from(value)
    }

    pub fn common_chord(&self, degree: u8) -> Note {
        assert!(degree > 0 && degree < 8, "Degree must be in [1, 6]");
        let new_tuning = self.modulation((degree - 1) as i8);
        let tonality = match degree {
            1 | 4 | 5 => Tonality::Major,
            2 | 3 | 6 => Tonality::Minor,
            _ => panic!("Invalid degree"),
        };

        Note { tuning: new_tuning, tonality, octave: 4, ..Default::default() }
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
struct Note {
    tuning: Tuning,
    tonality: Tonality,
    octave: u8,
    duration: f64,
    velocity: u8,
}

impl Default for Note {
    fn default() -> Self {
        Note {
            tuning: Tuning::C,
            tonality: Tonality::Major,
            octave: 4,
            duration: 0.25,
            velocity: 60,
        }
    }
}

impl Display for Note {
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

fn main() {
    let chord = Tuning::C;

    for degree in 1..=6 {
        println!("{:?}", chord.common_chord(degree).to_string());
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
    }
}
