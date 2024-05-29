mod device;
mod error;
mod keyboard;
mod led;

pub use error::{Error, Result};
pub use keyboard::{AlienKeyboard, Keycode};
pub use led::{Action, ActionEffect, AlienLED, LED};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        u32::from(color.red) << 16 | u32::from(color.green) << 8 | u32::from(color.blue)
    }
}
