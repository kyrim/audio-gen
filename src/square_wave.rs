use crate::traits::AudioSource;

#[derive(Clone)]
pub struct SquareWave {
    phase: f32,
    pub frequency: f32,
    sample_rate: u32,
}

impl SquareWave {
    /// Create a new SquareWave.
    pub fn new(sample_rate: u32, freq: f32) -> Self {
        Self {
            phase: 0.0,
            frequency: freq.min(sample_rate as f32 / 2.0),
            sample_rate,
        }
    }
}

impl AudioSource for SquareWave {
    fn next_sample(&mut self) -> f32 {
        // If phase is in the first half of [0..1), output +1.0.
        // Otherwise, output -1.0.
        let sample = if self.phase < 0.5 { 
            1.0 
        } else { 
            -1.0 
        };

        // Increment the phase.
        self.phase += self.frequency / self.sample_rate as f32;

        // Wrap phase back to [0..1).
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }

    fn set_frequency(&mut self, freq: f32) {
        // Limit to the Nyquist frequency.
        self.frequency = freq.min(self.sample_rate as f32 / 2.0);
    }
}
