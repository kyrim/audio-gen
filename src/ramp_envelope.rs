use crate::{stereo_sample::StereoSample, traits::AudioProcessor};

#[derive(Clone)]
pub struct RampEnvelope {
    // Current time in seconds since note-on
    current_time_s: f32,
    ramp_time_s: f32,
    // Sample rate
    sample_rate: f32,
}

impl RampEnvelope {
    pub fn new(sample_rate: f32, ramp_time_s: f32) -> Self {
        Self {
            current_time_s: 1.0,
            ramp_time_s,
            sample_rate,
        }
    }

    /// Called when the note first starts (Either first trigger, or retrigger).
    pub fn trigger(&mut self) {
        self.current_time_s = 0.0;
    }

    pub fn set_ramp(&mut self, ramp_time_s: f32) {
        self.ramp_time_s = ramp_time_s;
    }

    fn get_amount(&self) -> f32 {
        (self.current_time_s / self.ramp_time_s).min(1.0)
    }
}

impl AudioProcessor for RampEnvelope {
    fn process_sample(&mut self, input: StereoSample) -> StereoSample {
        // Get current amplitude
        let amount = self.get_amount();

        // Advance time by one sample
        self.current_time_s += 1.0 / self.sample_rate;

        StereoSample { left: input.left * amount, right: input.right * amount }
    }
}
