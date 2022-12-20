extern crate aych_delay;

use aych_delay::{Delay, Settings};
use rodio::{buffer::SamplesBuffer, Decoder, OutputStream, Sink};
use std::fs::File;
use std::path::Path;

fn main() {
    // Get the audio file name from the command line arguments,
    // or exit early if no file name was provided
    let file_name = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: cargo run --example basic <audio_file>");
        std::process::exit(1);
    });

    // Exit early if the audio file doesn't exist
    if !Path::new(&file_name).exists() {
        println!("File not found: {}", file_name);
        std::process::exit(1);
    }

    let banner = include_str!("../banner.txt");
    println!("{}", banner);

    let mut delay = Delay::new(Settings {
        delay_time: 166.66,
        feedback: 0.75,
        width: 0.5,
        lowpass_filter: 22000.0,
        highpass_filter: 300.0,
        dry_wet_mix: 0.5,
        output_level: 0.75,
        ..Settings::default()
    });

    let file = File::open(file_name).unwrap();
    let mut source = Decoder::new_looped(file).unwrap();

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Create a sink to play samples
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Create a 1024-sample buffer to hold the input and output data.
    let mut input = vec![0.0; 2048];
    let mut output = vec![0.0; 2048];

    loop {
        // Exit current loop early if the sink has enough samples to play
        if sink.len() >= 2 {
            continue;
        }

        // Fill the input buffer with samples from the WAV file
        for i in 0..input.len() {
            input[i] = source.next().unwrap_or(0) as f32 / 32768.0;
        }

        delay.process(&mut input, &mut output);

        // Play the output buffer
        sink.append(SamplesBuffer::new(2, 44100, output.clone()));
    }
}
