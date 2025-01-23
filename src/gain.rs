use crate::traits::AudioProcessor;

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
    fn process_sample(&mut self, input: f32) -> f32 {
        input * self.amount
    }
}