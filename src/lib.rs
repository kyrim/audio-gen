use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
 use traits::AudioSource;
use std::sync::Arc;

 mod sine_wave;
 mod saw_wave;
 mod square_wave;
 mod adsr_envelope;
 mod traits;

 mod polysynth;
 use polysynth::PolySynth;

 mod voice;
 mod gain;
 mod ramp_envelope;

mod editor;

pub struct PolySynthPlugin {
    params: Arc<PolySynthParams>,
    poly_synth: PolySynth
}

#[derive(Params)]
struct PolySynthParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "attack"]
    pub attack: FloatParam,

    #[id = "decay"]
    pub decay: FloatParam,

    #[id = "sustain"]
    pub sustain: FloatParam,

    #[id = "release"]
    pub release: FloatParam,

    #[id = "glide"]
    pub glide: FloatParam,
}

impl Default for PolySynthPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(PolySynthParams::default()),
            poly_synth: PolySynth::new(48000, 3)
        }
    }
}

impl Default for PolySynthParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            attack: FloatParam::new(
                "Attack",
                0.2,
                FloatRange::Linear { min: 0.0, max: 10.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),
            decay: FloatParam::new(
                "Decay",
                0.2,
                FloatRange::Linear { min: 0.0, max: 10.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),
            sustain: FloatParam::new(
                "Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" level"),
            release: FloatParam::new(
                "Release",
                0.2,
                FloatRange::Linear { min: 0.0, max: 10.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),
            glide: FloatParam::new(
                "Glide",
                0.1,
                FloatRange::Linear { min: 0.0, max: 10.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),
        }
    }
}

fn midi_note_to_frequency(note_number: u8) -> f32 {
    // A4 = note 69 = 440 Hz
    440.0 * 2.0_f32.powf((note_number as f32 - 69.0) / 12.0)
}

impl Plugin for PolySynthPlugin {
    const NAME: &'static str = "Kyrim's PolySynth";
    const VENDOR: &'static str = "Kyrim's Plugins GmbH";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        // Layout #1: Stereo out, no input
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        // Layout #2: Mono out, no input
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {

        // Pull in note events
        while let Some(event) = context.next_event() {

            match event {
                NoteEvent::NoteOn { timing: _, voice_id: _, channel:_, note, velocity:_ } => {
                    let freq = midi_note_to_frequency(note);
                    self.poly_synth.play(freq);
                }
                NoteEvent::NoteOff { timing:_, voice_id:_, channel:_, note, velocity:_ } => {
                    let freq = midi_note_to_frequency(note);
                    self.poly_synth.stop(freq);
                }
                _ => {}
            }
        }
    
        // Fill the audio buffer
        // For each sample index, `channels` is a slice where `channels[0]` is the left channel,
        // `channels[1]` is the right channel, etc.
        for channels in buffer.iter_samples() {

            self.poly_synth.set_attack(self.params.attack.smoothed.next());
            self.poly_synth.set_decay(self.params.decay.smoothed.next());
            self.poly_synth.set_sustain(self.params.sustain.smoothed.next());
            self.poly_synth.set_release(self.params.release.smoothed.next());
            self.poly_synth.set_glide(self.params.glide.smoothed.next());

            // Get the next sample from your synth/oscillator
            let next_out = self.poly_synth.next_sample();

            // Write that sample to all channels
            for sample_in_channel in channels {
                *sample_in_channel = next_out;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for PolySynthPlugin {
    const CLAP_ID: &'static str = "com.moist-plugins-gmbh.gain-gui-vizia";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Kyrim's PolySynth");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for PolySynthPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"GainGuiVIIIZIAAA";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(PolySynthPlugin);
nih_export_vst3!(PolySynthPlugin);