mod tptonepole;

pub use tptonepole::{TPTOnePole, TPTOnePoleStereo};

pub trait Filter {
    fn process(&mut self, input: f32) -> f32;
}

#[derive(Clone)]
pub enum Mode {
    LOWPASS,
    HIGHPASS,

    #[allow(dead_code)]
    ALLPASS,
}
