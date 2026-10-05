use std::time::Duration;

pub struct Envelope {
    i: usize,
    attack_samples: usize,
    decay_samples: usize,
    sustain_level: f32,
    release_samples: usize,
    release_start: Option<usize>,
}

impl Envelope {
    pub fn new(
        sample_rate: usize,
        attack_duration: Duration,
        decay_duration: Duration,
        sustain_level: f32,
        release_duration: Duration,
    ) -> Self {
        Self {
            i: 0,
            attack_samples: (attack_duration.as_secs_f32() * (sample_rate as f32)) as usize,
            decay_samples: (decay_duration.as_secs_f32() * (sample_rate as f32)) as usize,
            sustain_level,
            release_samples: (release_duration.as_secs_f32() * (sample_rate as f32)) as usize,
            release_start: None,
        }
    }

    pub fn release(&mut self) {
        if self.release_start.is_none() {
            self.release_start = Some(self.i);
        }
    }
}

impl Iterator for Envelope {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;

        if let Some(rel_start) = self.release_start {
            let num_release_samples = self.i - rel_start;
            if num_release_samples > self.release_samples {
                None
            } else {
                Some(self.sustain_level * ((self.release_samples - num_release_samples) as f32) / (self.release_samples as f32))
            }
        } else if self.i <= self.attack_samples {
            Some((self.i as f32) / (self.attack_samples as f32))
        } else if self.i <= (self.attack_samples + self.decay_samples) {
            let decay_i = (self.i - self.attack_samples) as f32;
            let decay_percentage = decay_i / (self.decay_samples as f32);
            Some(self.sustain_level + (1.0 - decay_percentage) * (1.0 - self.sustain_level))
        }  else {
            Some(self.sustain_level)
        } 
    }
}

// (1 - x) * (1 - l) + l = 1 - x + x*l = 1 + (l - 1) * x
