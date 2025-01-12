use crate::sine_wave::SineWave;
use crate::amp_adsr::AmpAdsr;
use crate::traits::{AudioSource, AudioProcessor};

#[derive(Clone)]
pub struct Voice {
    pub osc: SineWave,
    pub env: AmpAdsr,
    pub active: bool,
}

impl Voice {
    pub fn new(sample_rate: u32, freq: f32) -> Self {
        Self {
            osc: SineWave::new(sample_rate, freq),
            env: AmpAdsr::new(sample_rate as f32, 0.2, 0.2, 0.8, 0.5),
            active: false,
        }
    }

    pub fn play(&mut self, freq: f32) {
        self.osc.set_frequency(freq);
        self.osc.reset_phase(); // Reset the phase of the oscillator
        self.env.trigger();
        self.active = true;
    }

    pub fn stop(&mut self) {
        self.env.release();
    }

    pub fn get_frequency(&self) -> f32 {
        self.osc.frequency
    }

    pub fn next_sample(&mut self) -> f32 {
        if !self.active {
            return 0.0;
        }
        let raw = self.osc.next_sample();
        let out = self.env.process_sample(raw);

        // If envelope is effectively done
        if self.env.is_done() {
            self.active = false;
        }
        out
    }
}


#[derive(Clone)]
pub struct PolySynth {
    pub voices: Vec<Voice>,
}

impl PolySynth {
    pub fn new(sample_rate: u32, n_voices: usize) -> Self {
        let mut voices = Vec::new();
        for _ in 0..n_voices {
            voices.push(Voice::new(sample_rate, 220.0));
        }
        Self { voices }
    }

    pub fn play(&mut self, freq: f32) {
        // First, try to find an inactive voice
        if let Some(voice) = self.voices.iter_mut().find(|v| !v.active) {
            voice.play(freq); // Use the inactive voice
        } else {
            // No inactive voice: Find the closest frequency to `freq`
            let closest_voice = self.voices
                .iter_mut()
                .min_by(|v1, v2| {
                    (v1.get_frequency() - freq) 
                        .abs()
                        .partial_cmp(&(v2.get_frequency() - freq).abs())
                        .unwrap()
                });

            if let Some(voice) = closest_voice {
                voice.play(freq);
            }
        }
    }

    pub fn stop(&mut self, freq: f32) {
        self.voices
            .iter_mut()
            .filter(|v| v.active && v.get_frequency() == freq)
            .for_each(|v| v.stop());
    }
}

impl AudioSource for PolySynth {
    fn next_sample(&mut self) -> f32 {
        let mut sum = 0.0;
        for v in self.voices.iter_mut() {
            sum += v.next_sample();
        }
        sum
    }
}