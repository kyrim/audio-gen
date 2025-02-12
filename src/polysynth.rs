use crate::stereo_sample::StereoSample;
use crate::traits::AudioSource;
use crate::voice::Voice;

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

    pub fn set_attack(&mut self, attack_s: f32) {
        self.voices
            .iter_mut()
            .for_each(|v| v.env.set_attack(attack_s));
    }

    pub fn set_decay(&mut self, decay_s: f32) {
        self.voices
            .iter_mut()
            .for_each(|v| v.env.set_decay(decay_s));
    }

    pub fn set_sustain(&mut self, sustain_s: f32) {
        self.voices
            .iter_mut()
            .for_each(|v| v.env.set_sustain(sustain_s));
    }

    pub fn set_release(&mut self, release_s: f32) {
        self.voices
            .iter_mut()
            .for_each(|v| v.env.set_release(release_s));
    }

    pub fn set_glide(&mut self, glide_s: f32) {
        self.voices
            .iter_mut()
            .for_each(|v| v.frequency_env.set_ramp(glide_s));
    }
}

impl AudioSource for PolySynth {
    fn next_sample(&mut self) -> StereoSample {

        let mut stereo_sample = StereoSample { left: 0.0, right: 0.0 };

        for v in self.voices.iter_mut() {
            let next_sample = v.next_sample();
            stereo_sample.left += next_sample.left;
            stereo_sample.right += next_sample.right;
        }

        stereo_sample
    }
}