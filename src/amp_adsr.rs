use crate::traits::AudioProcessor;

#[derive(Clone)]
pub struct AmpAdsr {
    // Envelope parameters (in seconds)
    attack_s: f32,
    decay_s: f32,
    sustain_s: f32,
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

impl AmpAdsr {
    /// Create a new ADSR envelope.
    /// All times are in seconds: e.g., atk = 0.01, dec = 0.1, sus = 0.8, rel = 0.2
    pub fn new(sample_rate: f32, attack_s: f32, decay_s: f32, sustain_s: f32, release_s: f32) -> Self {
        Self {
            attack_s,
            decay_s,
            sustain_s,
            release_s,
            current_time_s: 0.0,
            is_released: false,
            release_start_time_s: 0.0,
            release_start_amp: 0.0,
            retrigger_start_amp: 0.0,
            sample_rate,
        }
    }

    /// Called when the note first starts (re-trigger).
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
 
        // Retrigger time is to avoid pops, when a voice is stolen.
        // On the retrigger, you may go from being in the middle of a envelope
        // to the start of the attack phase, a sharp change in envelope, causing a pop.
        let retrigger_time = 0.01; // 10ms
        let attack_time = retrigger_time + self.attack_s;
        let decay_time = attack_time + self.decay_s; // Attack + Decay

        if self.is_released {
            // Time since release triggered
            let time_releasing = self.current_time_s - self.release_start_time_s;
            // Fade from release_start_amp to 0 over 'release' seconds. Ensure we check that we don't go over 0.
            let ratio_released = (time_releasing / self.release_s).min(1.0);

            return self.release_start_amp * (1.0 - ratio_released);
        }
        
        if self.current_time_s <= retrigger_time {
            let ratio_retriggered =  (self.current_time_s / retrigger_time).min(1.0);
            
            return self.retrigger_start_amp * (1.0 - ratio_retriggered);
        }
        
        if self.current_time_s <= attack_time {
            // Get how far through attack we are in seconds
            let t_attk = self.current_time_s - retrigger_time;
            // Get a ratio so we can calculate progress
            return t_attk / self.attack_s;
        }
         
        if self.current_time_s <= decay_time {
            // Decay: amplitude from 1..sustain
            let t_dec = self.current_time_s - attack_time;
            let dec_ratio = t_dec / self.decay_s; // 0..1

            return 1.0 - (1.0 - self.sustain_s) * dec_ratio;
        }

        self.sustain_s
    }

    /// Returns `true` if the envelope has completely finished its release.
    pub fn is_done(&self) -> bool {
        // The envelope is done if we're in the released phase AND
        // the time since release started exceeds the release duration.
        self.is_released && (self.current_time_s - self.release_start_time_s) >= self.release_s
    }
}

impl AudioProcessor for AmpAdsr {
    fn process_sample(&mut self, input: f32) -> f32 {
        // Get current amplitude
        let amp = self.get_amplitude();

        // Advance time by one sample
        self.current_time_s += 1.0 / self.sample_rate;

        input * amp
    }
}
