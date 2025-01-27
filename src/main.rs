use nih_plug::prelude::*;
use audio_gen::PolySynthPlugin;

mod sine_wave;
mod saw_wave;
mod square_wave;
mod adsr_envelope;
mod traits;

mod polysynth;

mod voice;
mod gain;

mod ramp_envelope;

fn main() {
    nih_export_standalone::<crate::PolySynthPlugin>();
}
