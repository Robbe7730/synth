use std::{collections::HashMap, sync::{Arc, atomic::{AtomicBool, Ordering}}};

use crate::note::Note;

pub struct NoteManager {
    note_flags: HashMap<u8, Arc<AtomicBool>>
}

impl NoteManager {
    pub fn new() -> Self {
        NoteManager { 
            note_flags: HashMap::new()
        }
    }

    pub fn start(&mut self, key: u8, velocity: u8) -> Note {
        if self.note_flags.contains_key(&key) {
            self.note_flags.get(&key).unwrap().store(true, Ordering::SeqCst);
        }

        let flag = Arc::new(AtomicBool::new(false));
        self.note_flags.insert(key, flag.clone());
        Note::from_midi(key, velocity, flag)
    }

    pub fn stop(&mut self, key: u8) {
        let maybe_flag = self.note_flags.get(&key);

        if let Some(flag) = maybe_flag {
            flag.store(true, Ordering::SeqCst);
        }
    }
}
