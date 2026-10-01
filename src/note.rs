use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};

use rodio::{Sample, Source, source::{Amplify, SineWave}};

use crate::envelope::Envelope;

pub struct Note {
    inner: Amplify<SineWave>,
    envelope: Envelope,
    stop: Arc<AtomicBool>,
}

fn midi_key_to_freq(key: u8) -> f32 {
    return f32::powf(2.0, f32::from(i16::from(key) - 69) / 12.0) * 440.0;
}

impl Note {
    pub fn from_midi(key: u8, velocity: u8, stop: Arc<AtomicBool>) -> Self {
        Note { 
            inner: SineWave::new(midi_key_to_freq(key))
                .amplify(f32::from(velocity) / 64.0),
            envelope: Envelope::new(
                48000,
                Duration::from_millis(50),
                Duration::from_millis(300),
                0.5,
                Duration::from_millis(400),
            ),
            stop: stop,
        }
    }
}

impl Source for Note {
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

impl Iterator for Note {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop.load(Ordering::SeqCst) {
            self.envelope.release();
        }
        Some(self.inner.next()? * self.envelope.next()?)
    }
}
