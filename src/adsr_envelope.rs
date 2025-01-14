use core::panic;

use crate::traits::AudioProcessor;

#[derive(Clone)]
pub struct AdsrEnvelope {
    // Envelope parameters (_s = in seconds)
    attack_s: f32,
    decay_s: f32,
    sustain_level: f32, // Note: sustain isn't time based
    release_s: f32,

    // Current time in seconds since note-on
    current_time_s: f32,

    pub is_released: bool,
    // The time (in seconds) when release started
    release_start_time_s: f32,
    // The amplitude at the exact moment release started.
    release_start_amp: f32,

    // The amplitude at the exact moment retrigger started.
    retrigger_start_amp: f32,

    // Sample rate
    sample_rate: f32,
}

impl AdsrEnvelope {
    pub fn new(sample_rate: f32, attack_s: f32, decay_s: f32, sustain_level: f32, release_s: f32) -> Self {
        Self {
            attack_s,
            decay_s,
            sustain_level,
            release_s,
            current_time_s: 0.0,
            is_released: false,
            release_start_time_s: 0.0,
            release_start_amp: 0.0,
            retrigger_start_amp: 0.0,
            sample_rate,
        }
    }

    /// Called when the note first starts (Either first trigger, or retrigger).
    pub fn trigger(&mut self) {
        self.retrigger_start_amp = self.get_amplitude();
        self.current_time_s = 0.0;
        self.is_released = false;
        self.release_start_time_s = 0.0;
        self.release_start_amp = 0.0;
    }

    /// Called when the note ends (begin release stage).
    pub fn release(&mut self) {
        if !self.is_released {
            self.release_start_time_s = self.current_time_s;
            self.release_start_amp = self.get_amplitude();
            self.is_released = true;
        }
    }

    /// Return the current amplitude (0..1).
    pub fn get_amplitude(&self) -> f32 {
        if self.is_released {
            // Time since release triggered
            let time_releasing_s = self.current_time_s - self.release_start_time_s;
            // Fade from release_start_amp to 0 over 'release' seconds. Ensure we check that we don't go over 0.
            let ratio_released = time_releasing_s / self.release_s;

            if ratio_released > 1.0 {
                return 0.0;
            }

            return self.release_start_amp * (1.0 - ratio_released);
        }

        // Retrigger time is to avoid pops, when a voice is stolen.
        // On the retrigger, you may go from being in the middle of a envelope
        // to the start of the attack phase, a sharp change in envelope, causing a pop.
        // TODO: Something that would be really good is if the retrigger wasn't fixed, and rather was based on how
        // close to 0 it is, this may reduce latency.
        let retrigger_s = 0.01; // 10ms
        
        if self.current_time_s <= retrigger_s {
            let ratio_retriggered =  self.current_time_s / retrigger_s;
    
            if ratio_retriggered > 1.0 {
                panic!("ratio_retriggered of '{}' should not be more than 1, we are likely in the wrong stage", ratio_retriggered);
            }
            
            return self.retrigger_start_amp * (1.0 - ratio_retriggered);
        }
        
        let attack_end_time_s = retrigger_s + self.attack_s;

        if self.current_time_s <= attack_end_time_s {
            // Get how far through attack we are in seconds.
            let time_attacking_s = self.current_time_s - retrigger_s;
            // Get a ratio so we can calculate progress
            return time_attacking_s / self.attack_s;
        }

        let decay_end_time_s = attack_end_time_s + self.decay_s;

        if self.current_time_s <= decay_end_time_s {
            // Decay: amplitude from 1..sustain
            let time_decaying = self.current_time_s - attack_end_time_s;
            let decay_ratio = time_decaying / self.decay_s; // 0..1

            if decay_ratio > 1.0 {
                panic!("dec_ratio of '{}' should not be more than 1, we are likely in the wrong stage", decay_ratio);
            }

            return 1.0 - (1.0 - self.sustain_level) * decay_ratio;
        }

        self.sustain_level
    }

    /// Returns `true` if the envelope has completely finished its release.
    pub fn is_done(&self) -> bool {
        // The envelope is done if we're in the released phase AND
        // the time since release started exceeds the release duration.
        self.is_released && (self.current_time_s - self.release_start_time_s) >= self.release_s
    }
}

impl AudioProcessor for AdsrEnvelope {
    fn process_sample(&mut self, input: f32) -> f32 {
        // Get current amplitude
        let amp = self.get_amplitude();

        // Advance time by one sample
        self.current_time_s += 1.0 / self.sample_rate;

        input * amp
    }
}
