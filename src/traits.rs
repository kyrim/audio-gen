pub trait AudioSource {
    fn next_sample(&mut self) -> f32;
    fn set_frequency(&mut self, _freq: f32) {}
}

pub trait AudioProcessor {
    fn process_sample(&mut self, input: f32) -> f32;
}