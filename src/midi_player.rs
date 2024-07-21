use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

pub struct MidiPlayer {
    name: String,
    midi_out: Option<MidiOutput>,
    port: Option<MidiOutputPort>,
    midi_out_conn: Option<MidiOutputConnection>,
}

impl MidiPlayer {
    pub fn new(name: &str) -> Self {
        let midi_out = MidiOutput::new(name).ok();
        MidiPlayer {
            name: name.to_owned(),
            midi_out,
            port: None,
            midi_out_conn: None,
        }
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

    pub fn connect(&mut self, port_name: &str) -> Result<(), String> {
        match &self.port {
            None => Err("No port selected".to_owned()),
            Some(port) => {
                self.midi_out_conn = self
                    .midi_out
                    .take()
                    .ok_or("Midi output is not initialized")?
                    .connect(port, port_name)
                    .ok();
                Ok(())
            }
        }
    }

    pub fn play(&mut self, note: u8) {
        match &mut self.midi_out_conn {
            None => {}
            Some(conn) => {
                let _ = conn.send(&[0x90, note, 0x64]);
            }
        }
    }

    pub fn stop(&mut self, note: u8) {
        match &mut self.midi_out_conn {
            None => {}
            Some(conn) => {
                let _ = conn.send(&[0x80, note, 0x64]);
            }
        }
    }

    pub fn close(&mut self) {
        match self.midi_out_conn.take() {
            None => {}
            Some(conn) => {
                self.midi_out = Some(conn.close());
            }
        }
    }
}
