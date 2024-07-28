///
/// Duration represents the length of a note.
///
/// `real duration = 6.0 / duration`
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Duration {
    Whole = 6,
    Half = 12,
    Quarter = 24,
    Eighth = 48,
    Sixteenth = 96,
    ThirtySecond = 192,
    SixtyFourth = 384,
    HundredTwentyEighth = 768,

    // Dotted notes
    WholeDotted = 4,
    HalfDotted = 8,
    QuarterDotted = 16,
    EighthDotted = 32,
    SixteenthDotted = 64,
    ThirtySecondDotted = 128,
    SixtyFourthDotted = 256,
    HundredTwentyEighthDotted = 512,
}

impl From<u16> for Duration {
    fn from(value: u16) -> Self {
        match value {
            6 => Duration::Whole,
            12 => Duration::Half,
            24 => Duration::Quarter,
            48 => Duration::Eighth,
            96 => Duration::Sixteenth,
            192 => Duration::ThirtySecond,
            384 => Duration::SixtyFourth,
            768 => Duration::HundredTwentyEighth,
            // Dotted notes
            4 => Duration::WholeDotted,
            8 => Duration::HalfDotted,
            16 => Duration::QuarterDotted,
            32 => Duration::EighthDotted,
            64 => Duration::SixteenthDotted,
            128 => Duration::ThirtySecondDotted,
            256 => Duration::SixtyFourthDotted,
            512 => Duration::HundredTwentyEighthDotted,
            _ => panic!("Invalid value"),
        }
    }
}

impl From<Duration> for f32 {
    fn from(duration: Duration) -> f32 {
        6.0 / (duration as u16 as f32)
    }
}

impl From<Duration> for f64 {
    fn from(duration: Duration) -> f64 {
        f32::from(duration) as f64
    }
}

impl From<f32> for Duration {
    fn from(value: f32) -> Self {
        let duration = (6.0 * value) as u16;
        Duration::from(duration)
    }
}

impl From<f64> for Duration {
    fn from(value: f64) -> Self {
        Duration::from(value as f32)
    }
}

impl std::ops::Add<Duration> for Duration {
    type Output = f32;

    fn add(self, rhs: Duration) -> Self::Output {
        f32::from(self) + f32::from(rhs)
    }
}

impl Duration {
    pub fn with_dot(&self) -> Duration {
        match self {
            Duration::Whole => Duration::WholeDotted,
            Duration::Half => Duration::HalfDotted,
            Duration::Quarter => Duration::QuarterDotted,
            Duration::Eighth => Duration::EighthDotted,
            Duration::Sixteenth => Duration::SixteenthDotted,
            Duration::ThirtySecond => Duration::ThirtySecondDotted,
            Duration::SixtyFourth => Duration::SixtyFourthDotted,
            Duration::HundredTwentyEighth => Duration::HundredTwentyEighthDotted,
            _ => *self,
        }
    }
}
