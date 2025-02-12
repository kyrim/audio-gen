use crate::stereo_sample::StereoSample;

pub trait AudioSource {
    fn next_sample(&mut self) -> StereoSample;
    fn set_frequency(&mut self, _freq: f32) {}
}

pub trait AudioProcessor {
    fn process_sample(&mut self, input: StereoSample) -> StereoSample;
}