use crate::ramp_envelope::RampEnvelope;
use crate::traits::{AudioSource, AudioProcessor};
use crate::sine_wave::SineWave;
use crate::adsr_envelope::AdsrEnvelope;
use crate::gain::Gain;

#[derive(Clone)]
pub struct Voice {
    pub osc: SineWave,
    pub env: AdsrEnvelope,
    pub gain: Gain,
    frequency_env: RampEnvelope,
    start_frequency: f32,
    end_frequency: f32,
    pub active: bool,
}

impl Voice {
    pub fn new(sample_rate: u32, frequency: f32) -> Self {
        Self {
            osc: SineWave::new(sample_rate, frequency),
            env: AdsrEnvelope::new(sample_rate as f32, 0.02, 0.2, 1.0, 0.2),
            frequency_env: RampEnvelope::new(sample_rate as f32, 0.1),
            gain: Gain::new(0.9),
            active: false,
            start_frequency: frequency,
            end_frequency: frequency,
        }
    }

    pub fn play(&mut self, frequency: f32) {
        self.start_frequency = self.osc.frequency;
        self.end_frequency = frequency;
        self.frequency_env.trigger();
        self.env.trigger();
        self.active = true;
    }

    pub fn stop(&mut self) {
        self.env.release();
    }

    pub fn get_frequency(&self) -> f32 {
        self.end_frequency
    }

    pub fn next_sample(&mut self) -> f32 {
        if !self.active {
            return 0.0;
        }

        let freq = self.start_frequency + (self.frequency_env.process_sample(self.end_frequency - self.start_frequency));
        self.osc.set_frequency(freq);

        let raw = self.osc.next_sample();
        let osc_out = self.env.process_sample(raw);
        let gain_out = self.gain.process_sample(osc_out);

        // If envelope is effectively done
        if self.env.is_done() {
            self.active = false;
        }

        gain_out
    }
}