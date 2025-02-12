/// A simple stereo sample consisting of two f32 values: left and right.
#[derive(Copy, Clone, Debug)]
pub struct StereoSample {
    pub left: f32,
    pub right: f32,
}

impl StereoSample {
    /// Helper to sum left + right to mono
    pub fn to_mono(&self) -> f32 {
        0.5 * (self.left + self.right)
    }

    pub fn from_mono(mono: f32) -> Self {
        Self {
            left: mono,
            right: mono,
        }
    }
}
