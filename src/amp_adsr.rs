use crate::traits::AudioProcessor;

#[derive(Clone)]
pub struct AmpAdsr {
    // Envelope parameters (in seconds)
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,

    // Current time in seconds since note-on
    current_time: f32,

    // If we've released the note:
    pub released: bool,
    // The time (in seconds) when release was triggered
    release_start_time: f32,
    // The amplitude at the exact moment release was triggered
    release_start_amp: f32,

    // Sample rate
    sample_rate: f32,
}

impl AmpAdsr {
    /// Create a new ADSR envelope.
    /// All times are in seconds: e.g., atk = 0.01, dec = 0.1, sus = 0.8, rel = 0.2
    pub fn new(sample_rate: f32, attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            current_time: 0.0,
            released: false,
            release_start_time: 0.0,
            release_start_amp: 0.0,
            sample_rate,
        }
    }

    /// Called when the note first starts (re-trigger).
    pub fn trigger(&mut self) {
        self.current_time = 0.0;
        self.released = false;
        self.release_start_time = 0.0;
        self.release_start_amp = 0.0;
    }

    /// Called when the note ends (begin release stage).
    pub fn release(&mut self) {
        if !self.released {
            self.release_start_time = self.current_time;
            self.release_start_amp = self.get_amplitude();
            self.released = true;
        }
    }

    /// Return the current amplitude (0..1).
    pub fn get_amplitude(&self) -> f32 {
        // Break envelope into time regions
        let atk_time = self.attack;
        let dec_time = atk_time + self.decay; // Attack + Decay
        let rel_time = self.release;

        if self.released {
            // Time since release triggered
            let t_rel = self.current_time - self.release_start_time;
            // If we've fully finished releasing, amplitude is 0
            if t_rel >= rel_time {
                0.0
            } else {
                // Fade from release_start_amp to 0 over 'release' seconds
                let ratio = t_rel / rel_time;
                self.release_start_amp * (1.0 - ratio)
            }
        } else {
            // Attack -> Decay -> Sustain (before release)
            if self.current_time < atk_time {
                // Attack: amplitude 0..1
                self.current_time / atk_time
            } else if self.current_time < dec_time {
                // Decay: amplitude from 1..sustain
                let t_dec = self.current_time - atk_time;
                let dec_ratio = t_dec / self.decay; // 0..1
                1.0 - (1.0 - self.sustain) * dec_ratio
            } else {
                // Sustain
                self.sustain
            }
        }
    }

    /// Returns `true` if the envelope has completely finished its release.
    pub fn is_done(&self) -> bool {
        // The envelope is done if we're in the released phase AND
        // the time since release started exceeds the release duration.
        self.released && (self.current_time - self.release_start_time) >= self.release
    }
}

impl AudioProcessor for AmpAdsr {
    fn process_sample(&mut self, input: f32) -> f32 {
        // Get current amplitude
        let amp = self.get_amplitude();

        // Advance time by one sample
        self.current_time += 1.0 / self.sample_rate;

        input * amp
    }
}
