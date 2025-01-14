use crate::traits::AudioSource;

#[derive(Clone)]
pub struct SineWave {
    phase: f32,
    pub frequency: f32,
    sample_rate: u32,
}

impl SineWave {
    pub fn new(sample_rate: u32, freq: f32) -> Self {
        Self {
            phase: 0.0,
            frequency: freq,
            sample_rate,
        }
    }
}

impl AudioSource for SineWave {
    fn next_sample(&mut self) -> f32 {
        use std::f32::consts::PI;
        
        // Calculate the sample based on current phase in [0..1]
        let sample = (2.0 * PI * self.phase).sin();

        // Increment phase by freq / sample_rate
        self.phase += self.frequency / self.sample_rate as f32;

        // Wrap phase if it goes beyond 1.0
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }

    fn set_frequency(&mut self, freq: f32) {
        // To account for Nyquist theorem.
        // (The highest frequency component of a signal that can be accurately digitized is half the sampling rate)
        self.frequency = freq.min(self.sample_rate as f32 / 2.0);
    }
}
