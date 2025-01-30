#[derive(Debug, thiserror::Error)]
pub enum MusicError {
    #[error("Invalid pitch parameter")]
    InvalidPitch,

    #[error("Out of MIDI range: {0}")]
    MidiOutOfRange(u8),

    #[error("Unsupported chord types")]
    UnsupportedChord,

    #[error("Music Theory Conflict: {0}")]
    TheoryViolation(String),

    #[error("MIDI operation failed: {0}")]
    MidiError(String),

    #[error("Ratio of invalid legato: {actual}:{base}")]
    InvalidTupletRatio { actual: u8, base: u8 },

    #[error("Unsupported tuplet types")]
    UnsupportedTuplet,

    #[error("The duration of the tied notes does not match the note values.")]
    TupletDurationMismatch,
}
