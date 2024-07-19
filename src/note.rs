use std::fmt::Display;
use crate::tuning::Tuning;

#[derive(Copy, Clone, Debug)]
pub struct Note {
    pub chord: Tuning,
    pub octave: u8,
    pub(crate) duration: f32,
    pub(crate) velocity: f32,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chord_str = self.chord.to_string();
        let octave_str = self.octave.to_string();

        write!(f, "{}{}", chord_str, octave_str)
    }
}

impl Note {
    pub fn with_duration(self, duration: f32) -> Note {
        Note {
            chord: self.chord,
            octave: self.octave,
            duration,
            velocity: self.velocity,
        }
    }

    pub fn with_velocity(self, velocity: f32) -> Note {
        Note {
            chord: self.chord,
            octave: self.octave,
            duration: self.duration,
            velocity,
        }
    }
}