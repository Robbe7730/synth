use std::{error::Error, io::Write, sync::{Arc, Mutex}};

use midir::MidiInput;
use rodio::{mixer::Mixer, source::SineWave};

fn midi_key_to_freq(key: u8) -> f32 {
    return f32::powf(2.0, f32::from(i16::from(key) - 69) / 12.0) * 440.0;
}

fn midi_callback(_timestamp: u64, data: &[u8], mixer_mutex: &mut Arc<Mutex<Mixer>>) {
    let command: u8 = data[0];
    match command {
        0x80 => {
            let key = data[1];
            let velocity = data[2];
            println!("Note {} OFF with velocity {}", key, velocity);
        }
        0x90 => {
            let key = data[1];
            let velocity = data[2];
            let freq = midi_key_to_freq(key);
            println!("Note {} ON with velocity {} f={}", key, velocity, freq);
            let mixer = mixer_mutex.lock().unwrap();
            mixer.add(SineWave::new(freq));
        }
        _ => println!("Unknown MIDI command 0x{:02x}", command)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let mixer = stream_handle.mixer().to_owned();

    // Copied from https://github.com/Boddlnagg/midir/blob/master/src/lib.rs
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(midir::Ignore::None);
    let in_ports = midi_in.ports();
        let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    let mixer_mutex = Arc::new(Mutex::new(mixer));

    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        midi_callback,
        mixer_mutex,
    )?;

    loop {}
}
