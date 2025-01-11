use std::{sync::{Arc, Mutex}, time::Duration};

use crate::traits::AudioSource;

#[derive(Clone)]
pub struct RodioAdapter<T>
where
    T: AudioSource + Send + 'static,
{
    pub inner: Arc<Mutex<T>>,
    pub sample_rate: u32,
}

impl<T> RodioAdapter<T>
where
    T: AudioSource + Send + 'static,
{
    pub fn new(inner: Arc<Mutex<T>>, sample_rate: u32) -> Self {
        Self { inner, sample_rate }
    }
}

impl<T> Iterator for RodioAdapter<T>
where
    T: AudioSource + Send + 'static,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let mut locked = self.inner.lock().unwrap();
        Some(locked.next_sample())
    }
}

impl<T> rodio::Source for RodioAdapter<T>
where
    T: AudioSource + Send + 'static,
{
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1 // mono
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}