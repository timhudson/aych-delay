#![warn(missing_docs)]

//! A delay effect modelled after the H-Delay by Waves.
//!
//! Example:
//!
//! ```rust
//! use aych_delay::{Delay, Settings};
//!
//! let mut delay = Delay::new(Settings {
//!    delay_time: 166.66,
//!   feedback: 0.75,
//!   ..Settings::default()
//! });
//!
//! let input = vec![...];
//! let mut output = vec![0.0; 512];
//!
//! delay.process(&mut input, &mut output);
//! ```
//!

mod filters;
use filters::{Mode, TPTOnePoleStereo};

const SAMPLE_RATE: f32 = 44_100.0;

/// `Settings` contains the parameters for the delay effect.
pub struct Settings {
    /// The delay time in milliseconds.
    pub delay_time: f32,

    /// The output level of the delay effect. 1.0 is unity gain.
    pub output_level: f32,

    /// The feedback level of the delay effect (Also known as "decay").
    /// 0.0 is no feedback, 1.0 is infinite feedback.
    pub feedback: f32,

    /// Whether to use ping-pong delay.
    pub ping_pong: bool,

    /// The width of the delay effect, when ping-pong is enabled.
    /// 0.0 is mono, 1.0 is full stereo.
    pub width: f32,

    /// Whether to reverse the phase of the delayed signal.
    pub phase_reverse: bool,

    /// The cutoff frequency of the lowpass filter.
    pub lowpass_filter: f64,

    /// The cutoff frequency of the highpass filter.
    pub highpass_filter: f64,

    /// The dry/wet mix of the delay effect.
    pub dry_wet_mix: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            delay_time: 250.,
            output_level: 1.0,
            feedback: 0.8,
            ping_pong: true,
            width: 1.0,
            phase_reverse: true,
            lowpass_filter: 5000.0,
            highpass_filter: 500.0,
            dry_wet_mix: 0.5,
        }
    }
}

struct State {
    delay_buffer: Vec<(f32, f32)>,
    delay_buffer_index: usize,
    lowpass_filter: TPTOnePoleStereo,
    highpass_filter: TPTOnePoleStereo,
}

/// `Delay` is the main struct for the delay effect.
///
/// Internally, it maintains a buffer of delayed samples and a set of filters.
pub struct Delay {
    /// The current settings for the delay effect.
    pub settings: Settings,
    state: State,
}

impl Delay {
    /// Creates a new `Delay` instance with the specified settings.
    pub fn new(settings: Settings) -> Self {
        // Initialize the delay buffer with the specified delay time.
        let delay_buffer_size = (settings.delay_time / 1000.0) * SAMPLE_RATE;

        let state = State {
            delay_buffer: vec![(0.0, 0.0); delay_buffer_size as usize],
            delay_buffer_index: 0,
            lowpass_filter: TPTOnePoleStereo::new(
                Mode::LOWPASS,
                SAMPLE_RATE as f64,
                settings.lowpass_filter,
            ),
            highpass_filter: TPTOnePoleStereo::new(
                Mode::HIGHPASS,
                SAMPLE_RATE as f64,
                settings.highpass_filter,
            ),
        };

        Self { settings, state }
    }

    /// Processes the input buffer and writes the updated signal to the output buffer.
    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let mut input_index = 0;
        let mut output_index = 0;

        // Convert the input buffer into an array of stereo samples.
        let input_stereo: Vec<(f32, f32)> = input.chunks(2).map(|c| (c[0], c[1])).collect();

        while input_index < input_stereo.len() && output_index < output.len() {
            let input_sample = input_stereo[input_index];
            let delay_sample = self.state.delay_buffer[self.state.delay_buffer_index];

            // Apply feedback by scaling the delay sample by the current feedback level.
            let delay_sample = (
                delay_sample.0 * self.settings.feedback,
                delay_sample.1 * self.settings.feedback,
            );

            // Apply phase reverse by inverting the phase of the delay sample.
            let delay_sample = match self.settings.phase_reverse {
                true => (-delay_sample.0, -delay_sample.1),
                false => delay_sample,
            };

            // Apply filtering by convolving the delay sample with the filter coefficients.
            let delay_sample = self.state.lowpass_filter.process(delay_sample);
            let delay_sample = self.state.highpass_filter.process(delay_sample);

            // Apply ping-pong by mixing the left and right channels of the delay sample.
            if self.settings.ping_pong {
                let width = self.settings.width / 2.0 + 0.5;

                let pp_input = ((input_sample.0) * (1.0 - width), (input_sample.1) * width);

                let pp_delay = (
                    delay_sample.0 * (1.0 - width) + delay_sample.1 * width,
                    delay_sample.1 * (1.0 - width) + delay_sample.0 * width,
                );

                self.state.delay_buffer[self.state.delay_buffer_index] =
                    (pp_input.0 + pp_delay.0, pp_input.1 + pp_delay.1);
            } else {
                self.state.delay_buffer[self.state.delay_buffer_index] = (
                    input_sample.0 + delay_sample.0,
                    input_sample.1 + delay_sample.1,
                );
            }

            // Mix the dry and wet signals
            let delay_sample = (
                (1.0 - self.settings.dry_wet_mix) * input_sample.0
                    + self.settings.dry_wet_mix * delay_sample.0,
                (1.0 - self.settings.dry_wet_mix) * input_sample.1
                    + self.settings.dry_wet_mix * delay_sample.1,
            );

            // Apply output level by scaling the delayed sample by the current output level.
            let delay_sample = (
                delay_sample.0 * self.settings.output_level,
                delay_sample.1 * self.settings.output_level,
            );

            // Write the delayed sample to the output buffer.
            output[output_index * 2] = delay_sample.0;
            output[output_index * 2 + 1] = delay_sample.1;

            // Increment the input and output buffer indices.
            input_index += 1;
            output_index += 1;

            // Increment the delay buffer index and wrap around if necessary.
            self.state.delay_buffer_index =
                (self.state.delay_buffer_index + 1) % self.state.delay_buffer.len();
        }
    }
}
