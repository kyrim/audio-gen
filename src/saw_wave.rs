use crate::{stereo_sample::StereoSample, traits::AudioSource};

#[derive(Clone)]
pub struct SawWave {
    phase: f32,
    pub frequency: f32,
    sample_rate: u32,
}

impl SawWave {
    /// Create a new SawWave.
    pub fn new(sample_rate: u32, freq: f32) -> Self {
        Self {
            phase: 0.0,
            frequency: freq.min(sample_rate as f32 / 2.0),
            sample_rate,
        }
    }
}

impl AudioSource for SawWave {
    fn next_sample(&mut self) -> StereoSample {
        // The saw wave can be defined as going linearly from -1.0 to +1.0 over one period [0..1].
        // We'll map self.phase in [0..1] to the saw wave range of [-1..1].
        let sample = 2.0 * self.phase - 1.0;

        // Increment the phase by frequency / sample_rate.
        self.phase += self.frequency / self.sample_rate as f32;

        // Wrap phase if it goes beyond 1.0
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        StereoSample { left: sample, right: sample }
    }

    fn set_frequency(&mut self, freq: f32) {
        // Limit to Nyquist frequency (half the sample rate).
        self.frequency = freq.min(self.sample_rate as f32 / 2.0);
    }
}
