use crate::{stereo_sample::StereoSample, traits::AudioProcessor};

#[derive(Clone)]
pub struct Gain {
    amount: f32,
}

impl Gain {
    pub fn new(amount: f32) -> Self {
        Self {
            amount
        }
    }
}

impl AudioProcessor for Gain {
    fn process_sample(&mut self, input: StereoSample) -> StereoSample {
        StereoSample {  left: (input.left * self.amount),  right: (input.right * self.amount) }
    }
}