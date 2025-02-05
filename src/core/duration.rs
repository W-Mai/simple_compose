use super::MusicError;
use std::fmt::{Display, Formatter};

///
/// Duration represents the length of a note.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DurationBase {
    Maxima,       // 8
    Longa,        // 4
    Breve,        // 2
    Whole,        // 1
    Half,         // 1 / 2
    Quarter,      // 1 / 4
    Eighth,       // 1 / 8
    Sixteenth,    // 1 / 16
    ThirtySecond, // 1 / 32
    SixtyFourth,  // 1 / 64
}

/// Structure that represents a tuplet
#[derive(Debug, Clone, PartialEq)]
pub struct Tuplet {
    pub actual_notes: u8,            // The number of actual notes played
    pub base_notes: u8,              // Base number of notes
    pub base_duration: DurationBase, // Basic note value type
}

impl Tuplet {
    /// Creating a new legato configuration
    /// # Parameters
    /// - ratio: Actual note count/baseline note count (e.g., 3:2)
    /// - base: Basic note value type (such as Quarter, indicating a note value based on the quarter note)
    /// # Examples
    /// ```
    /// use simple_compose::{DurationBase, Tuplet};
    /// let triplet = Tuplet::new(3, 2, DurationBase::Quarter).unwrap();
    /// ```
    ///
    pub fn new(actual: u8, base: u8, duration: DurationBase) -> Result<Self, MusicError> {
        if actual == 0 || base == 0 {
            return Err(MusicError::InvalidTupletRatio { actual, base });
        }

        // Common Consonant Validity Check
        match (actual, base) {
            (3, 2) | (5, 4) | (6, 4) => Ok(()),
            _ if actual > base => Ok(()), // Allow unconventional but mathematically valid ligatures
            _ => Err(MusicError::UnsupportedTuplet),
        }?;

        Ok(Self {
            actual_notes: actual,
            base_notes: base,
            base_duration: duration,
        })
    }

    /// Calculate the legato ratio factor
    /// Return value: the time value correction coefficient for a single note
    pub fn ratio(&self) -> f32 {
        // For example, a 3:2 tritone returns 2.0/3.0.
        self.base_notes as f32 / self.actual_notes as f32
    }
}

#[derive(Debug, Clone)]
pub struct Duration {
    pub base: DurationBase,
    pub dots: u8, // [0, 3]
    pub tuplet: Option<Tuplet>,
}

impl Duration {
    /// Create basic time values
    pub fn new(base: DurationBase) -> Self {
        Self {
            base,
            dots: 0,
            tuplet: None,
        }
    }

    /// Add a dot (each dot extends the duration of the previous note by half).
    pub fn dotted(mut self, dots: u8) -> Self {
        self.dots = dots.min(3); // limited to 3 dots
        self
    }

    pub fn with_tuplet(mut self, tuplet: Tuplet) -> Self {
        self.tuplet = Some(tuplet);
        self
    }

    /// Calculate the actual duration value (unit: one quarter note equals one beat)
    pub fn in_quarters(&self) -> f32 {
        // Basic note value conversion (based on the quarter note)
        let base_value = match self.base {
            DurationBase::Maxima => 32.0,
            DurationBase::Longa => 16.0,
            DurationBase::Breve => 8.0,
            DurationBase::Whole => 4.0,
            DurationBase::Half => 2.0,
            DurationBase::Quarter => 1.0,
            DurationBase::Eighth => 0.5,
            DurationBase::Sixteenth => 0.25,
            DurationBase::ThirtySecond => 0.125,
            DurationBase::SixtyFourth => 0.0625,
        };

        // Calculate the extension of the dot accent.
        let dotted_value = (0..self.dots).fold(base_value, |acc, _| acc + acc / 2.0);

        // Applicative tuplet ratio
        match &self.tuplet {
            Some(t) => {
                // Check if the reference value matches.
                if t.base_duration != self.base {
                    // In practical application, an error should be returned, but here we simplify the processing.
                    panic!("Tuplet base duration mismatch");
                }
                dotted_value * t.ratio()
            }
            None => dotted_value,
        }
    }

    /// Converts a given `f32` beat value to a `Duration` structure.
    pub fn from_quarters(value: f32) -> Self {
        // Define the base durations and their corresponding values
        let duration_bases: Vec<(DurationBase, f32)> = vec![
            (DurationBase::Maxima, 8.0),
            (DurationBase::Longa, 4.0),
            (DurationBase::Breve, 2.0),
            (DurationBase::Whole, 1.0),
            (DurationBase::Half, 0.5),
            (DurationBase::Quarter, 0.25),
            (DurationBase::Eighth, 0.125),
            (DurationBase::Sixteenth, 0.0625),
            (DurationBase::ThirtySecond, 0.03125),
            (DurationBase::SixtyFourth, 0.015625),
        ];

        // Try to match the value to a base duration
        let (base, base_value) = duration_bases
            .iter()
            .find(|(_, v)| (value - *v * 4.0).abs() < f32::EPSILON) // Allow a small floating-point tolerance
            .map(|(b, v)| (*b, *v * 4.0))
            .unwrap_or((DurationBase::Quarter, 1.0)); // Default to Whole if no match

        // Calculate dots if the value is not exactly matching a base duration
        let mut dots = 0;
        let mut remaining = value - base_value;
        while remaining > 0.0 && dots < 3 {
            remaining -= base_value / (2.0f32.powi(dots as i32));
            dots += 1;
        }

        // Now is always set to None.
        let tuplet = None;

        Duration { base, dots, tuplet }
    }

    /// Conversion to seconds (considering BPM)
    /// # Parameters
    /// - tempo: Beat tempo (BPM, quarter notes per minute)
    pub fn in_seconds(&self, tempo: f32) -> f32 {
        let quarters = self.in_quarters();
        (60.0 / tempo) * quarters
    }
}

impl From<Duration> for f32 {
    fn from(duration: Duration) -> f32 {
        duration.in_quarters()
    }
}

impl From<Duration> for f64 {
    fn from(duration: Duration) -> f64 {
        duration.in_quarters() as f64
    }
}

impl From<&Duration> for f32 {
    fn from(duration: &Duration) -> f32 {
        duration.in_quarters()
    }
}

impl From<&Duration> for f64 {
    fn from(duration: &Duration) -> f64 {
        duration.in_quarters() as f64
    }
}

impl From<f32> for Duration {
    fn from(value: f32) -> Self {
        Duration::from_quarters(value)
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

impl std::ops::Add<f32> for Duration {
    type Output = f32;

    fn add(self, rhs: f32) -> Self::Output {
        f32::from(self) + rhs
    }
}

impl std::ops::Add<f64> for Duration {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        f64::from(self) + rhs
    }
}

impl std::ops::Add<Duration> for f32 {
    type Output = f32;

    fn add(self, rhs: Duration) -> Self::Output {
        self + f32::from(rhs)
    }
}

impl std::ops::Add<Duration> for f64 {
    type Output = f64;

    fn add(self, rhs: Duration) -> Self::Output {
        self + f64::from(rhs)
    }
}

impl std::ops::Add<&Duration> for f64 {
    type Output = f64;

    fn add(self, rhs: &Duration) -> Self::Output {
        self + f64::from(rhs)
    }
}

impl std::ops::AddAssign<Duration> for f32 {
    fn add_assign(&mut self, rhs: Duration) {
        *self += f32::from(rhs);
    }
}

impl std::ops::AddAssign<Duration> for f64 {
    fn add_assign(&mut self, rhs: Duration) {
        *self += f64::from(rhs);
    }
}

impl std::ops::AddAssign<&Duration> for f64 {
    fn add_assign(&mut self, rhs: &Duration) {
        *self += f64::from(rhs);
    }
}

impl std::ops::AddAssign<f32> for Duration {
    fn add_assign(&mut self, rhs: f32) {
        *self = Duration::from(self.in_quarters() + rhs);
    }
}

impl std::ops::AddAssign<f64> for Duration {
    fn add_assign(&mut self, rhs: f64) {
        *self = Duration::from(self.in_quarters() as f64 + rhs);
    }
}

impl Display for DurationBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationBase::Maxima => write!(f, "𝅜𝅜𝅜𝅜"),
            DurationBase::Longa => write!(f, "𝅜𝅜"),
            DurationBase::Breve => write!(f, "𝅜"),
            DurationBase::Whole => write!(f, "𝅝"),
            DurationBase::Half => write!(f, "𝅗𝅥"),
            DurationBase::Quarter => write!(f, "𝅘𝅥"),
            DurationBase::Eighth => write!(f, "𝅘𝅥𝅮"),
            DurationBase::Sixteenth => write!(f, "𝅘𝅥𝅯"),
            DurationBase::ThirtySecond => write!(f, "𝅘𝅥𝅰"),
            DurationBase::SixtyFourth => write!(f, "𝅘𝅥𝅱"),
        }
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let duration_base = &self.base;
        write!(
            f,
            "{}{}",
            duration_base,
            "𝅭".to_string().repeat(self.dots as usize)
        )
    }
}

pub mod duration_utils {
    use super::Duration;
    use crate::DurationBase;
    use rand::prelude::*;

    pub fn generate_one_measure(beat: u8) -> Vec<Duration> {
        let beat = beat as f64;
        let mut durations = vec![];
        let mut rng = thread_rng();
        let mut duration_sum = 0.0;
        while duration_sum < beat {
            let duration_base = *[DurationBase::Half, DurationBase::Quarter]
                .choose(&mut rng)
                .unwrap();
            let duration = &Duration::new(duration_base);
            if duration_sum + duration > beat {
                break;
            }
            duration_sum += duration;
            durations.push(duration.clone());
        }

        let remainder = beat - duration_sum;
        if remainder > 0.0 {
            durations.push(remainder.into());
        }
        durations
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_duration() {
        let q = Duration::new(DurationBase::Quarter);
        assert_eq!(q.in_quarters(), 1.0);
    }

    #[test]
    fn dotted_note() {
        let d = Duration::new(DurationBase::Quarter).dotted(1);
        assert_eq!(d.in_quarters(), 1.0 * 1.5);
        assert_eq!(
            d.in_quarters(),
            Duration::from_quarters(d.in_quarters()).in_quarters()
        )
    }

    #[test]
    fn triplet() {
        let triplet = Tuplet::new(3, 2, DurationBase::Quarter).unwrap();
        let note = Duration::new(DurationBase::Quarter).with_tuplet(triplet);

        // The triple note takes up 2/3 of a quarter note.
        assert_eq!(note.in_quarters(), 2.0 / 3.0);
    }

    #[test]
    fn complex_case() {
        // Dotted eighth note + quintuplet note
        let tuplet = Tuplet::new(5, 4, DurationBase::Eighth).unwrap();
        let note = Duration::new(DurationBase::Eighth)
            .dotted(1)
            .with_tuplet(tuplet);

        // Basic note value: 0.5 (quaver) + 0.25 (dotted) = 0.75
        // Applied tuplet ratio: 4/5
        assert_eq!(note.in_quarters(), 0.75 * (4.0 / 5.0));
    }
}
