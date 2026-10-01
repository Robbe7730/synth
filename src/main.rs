use std::error::Error;

use rodio::source::SineWave;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let mixer = stream_handle.mixer();

    mixer.add(
        SineWave::new(440.0)
    );

    loop {}
}
