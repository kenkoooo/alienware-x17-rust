use alienfx_x17::{AlienKeyboard, AlienLED};
use rusb::Context;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut context = Context::new()?;
    let keyboard = AlienKeyboard::new(&mut context)?;
    keyboard.rainbow_wave()?;

    let led = AlienLED::new(&mut context)?;
    led.rainbow_wave()?;

    Ok(())
}
