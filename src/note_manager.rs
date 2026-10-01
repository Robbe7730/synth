use std::sync::{Arc, atomic::AtomicBool};

use crate::note_source::NoteSource;

pub struct NoteManager {}

impl NoteManager {
    pub fn new() -> Self {
        NoteManager {  }
    }

    pub fn start(&self, key: u8, velocity: u8) -> NoteSource {
        NoteSource::from_midi(key, velocity, Arc::new(AtomicBool::new(false)))
    }
}
