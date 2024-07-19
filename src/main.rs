use std::thread::sleep;
use std::time::Duration;
use rand::prelude::*;
use midir::MidiOutput;

use simple_compose::*;

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

///
/// ```plaintext
/// Tuning ----------------> Chord -------------> Note
///         common_chord()          breakdown()
/// ```
fn main() {
    let tuning = Tuning::C;
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

                play_note(note.chord as u8 + (note.octave + 1) * 12 - 1, (duration * 8.0) as u64)
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
