use std::sync::{Arc, atomic::{AtomicBool, Ordering::Relaxed}};

use rodio::{Sample, Source, source::{SineWave, Stoppable}};

pub struct NoteSource {
    inner: Stoppable<SineWave>,
    stop: Arc<AtomicBool>,
}

fn midi_key_to_freq(key: u8) -> f32 {
    return f32::powf(2.0, f32::from(i16::from(key) - 69) / 12.0) * 440.0;
}

impl NoteSource {
    pub fn from_midi(key: u8, _velocity: u8, stop: Arc<AtomicBool>) -> Self {
        NoteSource { 
            inner: SineWave::new(midi_key_to_freq(key)).stoppable(),
            stop: stop,
        }
    }
}

impl Source for NoteSource {
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }

    fn channels(&self) -> rodio::ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.inner.total_duration()
    }
}

impl Iterator for NoteSource {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop.load(Relaxed) {
            self.inner.stop();
            None
        } else {
            self.inner.next()
        }
    }
}
