use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;
use rand::prelude::*;
use midir::{MidiOutput, MidiOutputPort};

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

// 按照 1/32 1/16 1/8 1/4 1/2 1 2 4 的时值随机生成一个时值序列，并保证生成的时值序列长度为 4
fn generate_duration() -> Vec<f32> {
    let mut durations = vec![];
    let mut rng = thread_rng();
    let mut duration_sum = 0.0;
    while duration_sum < 4.0 {
        let duration = match rng.gen_range(3..=5) {
            0 => 1.0 / 32.0,
            1 => 1.0 / 16.0,
            2 => 1.0 / 8.0,
            3 => 1.0 / 4.0,
            4 => 1.0 / 2.0,
            5 => 1.0,
            6 => 2.0,
            7 => 4.0,
            _ => { unreachable!(); }
        };
        if duration_sum + duration > 4.0 {
            break;
        }
        duration_sum += duration;
        durations.push(duration);
    }

    let remainder = duration_sum - 4.0;
    if remainder > 0.0 {
        durations.push(remainder);
    }
    durations
}
fn main() {
    let tuning = Tuning::from(1);
    let chords = vec![
        tuning.common_chord(1),
        tuning.common_chord(6),
        tuning.common_chord(4),
        tuning.common_chord(5),
        tuning.common_chord(1),
        tuning.common_chord(6),
        tuning.common_chord(4),
        tuning.common_chord(5),
        tuning.common_chord(1),
    ];

    let mut rng = thread_rng();

    let midi_out = MidiOutput::new("My Test Output").unwrap();

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();

    let need_play = out_ports.len() > 0;

    let out_port = if need_play {
        Some(match out_ports.len() {
            1 => {
                println!(
                    "Choosing the only available output port: {}",
                    midi_out.port_name(&out_ports[0]).unwrap()
                );
                &out_ports[0]
            }
            _ => {
                println!("\nAvailable output ports:");
                for (i, p) in out_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_out.port_name(p).unwrap());
                }
                print!("Please select output port: ");
                let mut input = String::new();
                out_ports
                    .get(input.trim().parse::<usize>().unwrap())
                    .ok_or("invalid output port selected").unwrap()
            }
        })
    } else { None };


    println!("\nOpening connection");
    let mut conn_out = if need_play { Some(midi_out.connect(out_port.unwrap(), "midir-test").unwrap()) } else { None };
    println!("Connection open. Listen!");
    // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
    let mut play_note = |note: u8, duration: u64| {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x64;
        // We're ignoring errors in here
        if let Some(conn_out) = conn_out.as_mut() {
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            sleep(Duration::from_millis(duration * 80));
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        }
    };
    {
        for chord in chords {
            let notes = chord.breakdown(4);
            let durations = generate_duration();
            for duration in durations {
                let note = notes.choose(&mut rng).unwrap().clone();
                let note = note.with_duration(duration);
                print!("{}[{}] ", note, duration);

                play_note(match note.chord {
                    Tuning::None => 0,
                    Tuning::C => 1,
                    Tuning::D => 3,
                    Tuning::E => 5,
                    Tuning::F => 6,
                    Tuning::G => 8,
                    Tuning::A => 10,
                    Tuning::B => 11,
                } + (note.octave + 1) * 12 - 1, (duration * 8.0) as u64)
            }
            println!("|");
        }
    }

    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    if let Some(conn_out) = conn_out {
        conn_out.close();
    }
    println!("Connection closed");
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
