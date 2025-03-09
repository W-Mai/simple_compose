use crate::{Measure, MusicError, Score, Tuning};
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use std::array;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MidiPlayer {
    name: String,
    midi_out: Option<MidiOutput>,
    port: Option<MidiOutputPort>,
    midi_out_conn: Rc<RefCell<Option<MidiOutputConnection>>>,

    midi_player_channels: Option<[RefCell<MidiPlayerChannel>; 16]>,
}

pub struct MidiPlayerChannel {
    midi_out_conn: Rc<RefCell<Option<MidiOutputConnection>>>,
    channel: u8,
}

impl MidiPlayer {
    pub fn new(name: &str) -> Self {
        let mut midi_player = MidiPlayer {
            name: name.to_owned(),
            midi_out: None,
            port: None,
            midi_out_conn: Rc::new(RefCell::new(None)),
            midi_player_channels: None,
        };

        let midi_out = MidiOutput::new(&midi_player.name).ok();
        midi_player.midi_out = midi_out;
        midi_player
    }

    pub fn list_ports(&self) -> Vec<String> {
        if let Some(midi_out) = &self.midi_out {
            let midi_out_ports = midi_out.ports();
            midi_out_ports
                .iter()
                .filter_map(|port| midi_out.port_name(port).ok())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn select_port(&mut self, port_index: usize) -> Result<(), String> {
        if let Some(midi_out) = &self.midi_out {
            let midi_out_ports = midi_out.ports();
            if port_index >= midi_out_ports.len() {
                return Err("Port index out of range".to_owned());
            }
            self.port = Some(midi_out_ports[port_index].to_owned());
        } else {
            return Err("No MIDI output".to_owned());
        }
        Ok(())
    }

    pub fn connect(
        &mut self,
        port_name: &str,
    ) -> Result<&mut [RefCell<MidiPlayerChannel>; 16], String> {
        match &self.port {
            None => Err("No port selected".to_owned()),
            Some(port) => {
                self.midi_out_conn = Rc::new(RefCell::new(
                    self.midi_out
                        .take()
                        .ok_or("Midi output is not initialized")?
                        .connect(port, port_name)
                        .ok(),
                ));

                self.midi_player_channels = Some(array::from_fn(|i| {
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), i as u8))
                }));
                Ok(self.midi_player_channels.as_mut().unwrap())
            }
        }
    }

    pub fn close(&mut self) {
        self.reset_notes();
        match self.midi_out_conn.take() {
            None => {}
            Some(conn) => {
                self.midi_out = Some(conn.close());
            }
        }
    }

    fn reset_notes(&mut self) {
        for channel in self.midi_player_channels.iter_mut().flatten() {
            channel.borrow_mut().stop_all();
        }
    }
}

impl Drop for MidiPlayer {
    fn drop(&mut self) {
        self.close();
    }
}

impl<'a> MidiPlayerChannel {
    fn new(midi_out_conn: Rc<RefCell<Option<MidiOutputConnection>>>, channel: u8) -> Self {
        MidiPlayerChannel {
            midi_out_conn,
            channel,
        }
    }

    pub fn play_notes(&mut self, notes: &[u8]) {
        self.midi_out_conn.borrow_mut().as_mut().map(|conn| {
            notes.iter().for_each(|note| {
                let _ = conn.send(&[0x90 | (self.channel & 0xF), *note, 0x64]);
            })
        });
    }

    pub fn stop_notes(&mut self, notes: &[u8]) {
        self.midi_out_conn.borrow_mut().as_mut().map(|conn| {
            notes.iter().for_each(|note| {
                let _ = conn.send(&[0x80 | (self.channel & 0xF), *note, 0x64]);
            })
        });
    }

    pub fn stop_all(&mut self) {
        self.stop_notes(&array::from_fn::<_, 128, _>(|i| i as u8));
    }
}

impl Tuning {
    /// Get MIDI pitch number (A4=69)
    pub fn midi_number(&self) -> Result<u8, MusicError> {
        let base = self.class as u8;
        if base == 0 {
            return Ok(0);
        }
        let base = base - 1;
        let num = (self.octave + 1) * 12 + base as i8;
        num.try_into().map_err(|_| MusicError::InvalidPitch)
    }
}

impl MidiPlayer {
    /// Play a score
    ///
    /// TODO: optimize the performance
    pub fn play_score<const TRACK_COUNT: usize>(
        &mut self,
        score: Score<TRACK_COUNT>,
    ) -> Result<(), String> {
        use std::time;

        let tracks = score.get_tracks();
        tracks.first().ok_or("No tracks in score".to_owned())?;

        self.list_ports()
            .first()
            .ok_or("No MIDI output ports available".to_owned())?;
        self.select_port(0)?;
        let channels = self.connect("Simple Compose Port 0")?;
        let max_track_count = TRACK_COUNT.min(channels.len());

        let tempo = score.tempo();
        let beats_per_measure = score.time_signature().0;
        let beat_duration = time::Duration::from_secs_f64(60.0 / tempo as f64);
        let measure_duration = beat_duration * beats_per_measure as u32;

        struct TimedEvent {
            trigger_time: time::Duration,
            track_idx: usize,
            notes: Vec<u8>,
            is_start: bool,
        }

        let mut events = Vec::new();
        for (track_idx, track) in tracks[..max_track_count].iter().enumerate() {
            for (measure_idx, measure) in track.get_measures().iter().enumerate() {
                match measure {
                    Measure::Rest => {}
                    Measure::Chord(chord) => {
                        let chord_notes: Vec<u8> = chord
                            .components()
                            .iter()
                            .map(|t| t.midi_number().unwrap())
                            .collect();

                        let start_time = measure_duration * measure_idx as u32;
                        let end_time = start_time + measure_duration;

                        events.push(TimedEvent {
                            trigger_time: start_time,
                            track_idx,
                            notes: chord_notes.clone(),
                            is_start: true,
                        });
                        events.push(TimedEvent {
                            trigger_time: end_time,
                            track_idx,
                            notes: chord_notes,
                            is_start: false,
                        });
                    }
                    Measure::Note(notes) => {
                        let mut current_start = 0.0;
                        for note in notes {
                            let duration = note.duration();
                            let start = current_start;
                            current_start += duration.in_quarters();
                            let note_start = measure_duration * measure_idx as u32
                                + beat_duration.mul_f64(start as f64);
                            let note_end =
                                note_start + beat_duration.mul_f64(duration.in_quarters() as f64);

                            let midi_num = note.tuning().midi_number().unwrap();

                            events.push(TimedEvent {
                                trigger_time: note_start,
                                track_idx,
                                notes: vec![midi_num],
                                is_start: true,
                            });
                            events.push(TimedEvent {
                                trigger_time: note_end,
                                track_idx,
                                notes: vec![midi_num],
                                is_start: false,
                            });
                        }
                    }
                }
            }
        }

        events.sort_by(|a, b| a.trigger_time.cmp(&b.trigger_time));
        let time_start = time::SystemTime::now();
        for event in events {
            let trigger_moment = time_start + event.trigger_time;
            let now = time::SystemTime::now();

            if let Ok(wait_duration) = trigger_moment.duration_since(now) {
                std::thread::sleep(wait_duration);
            }
            let channel = &channels[event.track_idx];
            println!(
                "Playing notes {:?} on channel {}",
                event.notes, event.track_idx
            );
            if event.is_start {
                channel.borrow_mut().play_notes(&event.notes);
            } else {
                channel.borrow_mut().stop_notes(&event.notes);
            }
        }

        Ok(())
    }
}
