// "Topology preserving transform" one-pole filter.
//
// Derived from the soul-lang project (ISC license),
// which is derived from work by Zavalishin and Pirkle.
// https://github.com/soul-lang/SOUL

use crate::filters::{Filter, Mode};
use std::f64::consts::PI;

const MIN_FREQ: f64 = 5.0;
const MAX_FREQ: f64 = 22000.0;
const NORMALIZED_FREQ_LIMIT: f64 = 0.49;

fn get_coefficient(sample_rate: f64, freq_hz: f64) -> f64 {
    let wd = 2.0 * PI * freq_hz;
    let t = 1.0 / sample_rate;
    let wa = (2.0 / t) * (wd * t / 2.0).tan();
    let g = wa * t / 2.0;

    g / (1.0 + g)
}

pub struct TPTOnePole {
    mode: Mode,
    b: f64,
    z1: f32,
}

impl TPTOnePole {
    pub fn new(mode: Mode, sample_rate: f64, freq_hz: f64) -> Self {
        // Clamp the frequency to the Nyquist frequency
        let freq_hz = freq_hz.max(MIN_FREQ).min(MAX_FREQ * NORMALIZED_FREQ_LIMIT);

        Self {
            mode,
            b: get_coefficient(sample_rate, freq_hz),
            z1: 0.0,
        }
    }

    fn process_lpf(&mut self, input: f32) -> f32 {
        let vn = (input - self.z1) * self.b as f32;
        let lpf = vn + self.z1;
        self.z1 = vn + lpf;

        lpf
    }

    fn process_hpf(&mut self, input: f32) -> f32 {
        input - self.process_lpf(input)
    }

    fn process_apf(&mut self, input: f32) -> f32 {
        let lpf = self.process_lpf(input);
        let hpf = input - lpf;

        lpf - hpf
    }
}

impl Filter for TPTOnePole {
    fn process(&mut self, input: f32) -> f32 {
        match self.mode {
            Mode::LOWPASS => self.process_lpf(input),
            Mode::HIGHPASS => self.process_hpf(input),
            Mode::ALLPASS => self.process_apf(input),
        }
    }
}

pub struct TPTOnePoleStereo {
    left: TPTOnePole,
    right: TPTOnePole,
}

impl TPTOnePoleStereo {
    pub fn new(mode: Mode, sample_rate: f64, freq_hz: f64) -> Self {
        Self {
            left: TPTOnePole::new(mode.clone(), sample_rate, freq_hz),
            right: TPTOnePole::new(mode.clone(), sample_rate, freq_hz),
        }
    }

    pub fn process(&mut self, input: (f32, f32)) -> (f32, f32) {
        (self.left.process(input.0), self.right.process(input.1))
    }
}
