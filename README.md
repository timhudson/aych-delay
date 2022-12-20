# Aych-Delay

Aych-Delay is a delay effect modelled after the H-Delay by Waves. It applies a delay effect to audio data and includes features such as feedback, ping-pong, width control, and lowpass/highpass filtering.

## Disclaimer

This project came about as a way for me to scratch an itch. I've been wanting to write audio software with Rust for a while, and recreating the H-Delay seemed like a fun challenge. I'm NOT a DSP expert, and I'm sure there are many ways in which this code could be improved. If you have any suggestions, feel free to open an issue or submit a pull request.

## Installation

To use Aych-Delay in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
aych_delay = "0.1.1"
```


## Usage

To use Aych-Delay, create a new instance of the `Delay` struct with the desired settings and call the `process` method on your audio data:

```rust
use aych_delay::{Delay, Settings};

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

let mut delay = Delay::new(settings);

let input: &[f32] = &[...];
let mut output: &mut [f32] = &mut [...];

delay.process(input, output);
```

You can also use the `default()` method of the `Settings` struct to get a set of default values:

```rust
let settings = Settings::default();
let mut delay = Delay::new(settings);
```

## Examples

The `examples` directory contains a basic example of using Aych-Delay with the `rodio` library to play a sound file with the plugin applied. To run the example, use the following command:

```bash
cargo run --example basic <audio_file>
```

## Filters

Aych-Delay includes lowpass and highpass filters implemented with the `TPTOnePoleStereo` struct. This is a "Topology preserving transform" one-pole filter, derived from work by Zavalishin and Pirkle, and an implementation of the filter within the [SOUL](https://github.com/soul-lang/SOUL) project (ISC license).

## License

Aych-Delay is licensed under the MIT license. See the `LICENSE` file for more information.
