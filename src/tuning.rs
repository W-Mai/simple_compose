use std::fmt::Display;
use crate::chord::Chord;
use crate::tonality::Tonality;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
#[derive(PartialEq)]
pub enum Tuning {
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