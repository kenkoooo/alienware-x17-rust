use std::time::Duration;

use rusb::UsbContext;

use crate::{device::AlienDeviceInner, Color, Result};

pub struct AlienKeyboard<T: UsbContext> {
    device: AlienDeviceInner<T>,
}

impl<T: UsbContext> AlienKeyboard<T> {
    pub fn new(context: &mut T) -> Result<AlienKeyboard<T>> {
        Ok(AlienKeyboard {
            device: AlienDeviceInner::new(context, 0x0D62, 0xCABC)?,
        })
    }

    fn write_packet(&self, packet: &[u8]) -> Result<()> {
        self.device
            .handle
            .write_control(0x21, 0x09, 0x3cc, 0x0, packet, Duration::from_secs(5))?;
        Ok(())
    }

    pub fn rainbow_wave(&self) -> Result<()> {
        let bytes = hex_to_bytes(
            r"cc800305000001010101000000000000050000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        )?;
        self.write_packet(&bytes)
    }

    pub fn change_color(&self, key: Keycode, color: Color) -> Result<()> {
        let bytes = key.bytes(color)?;
        self.write_packet(&bytes)
    }
}

fn hex_to_bytes(s: &str) -> Result<Vec<u8>> {
    if s.len() % 2 == 0 {
        let mut bytes = vec![];
        for i in (0..s.len()).step_by(2) {
            let s = &s[i..i + 2];
            let b = u8::from_str_radix(s, 16).map_err(|_| crate::Error::InvalidHex(s.into()))?;
            bytes.push(b);
        }
        Ok(bytes)
    } else {
        Err(crate::Error::InvalidHex(s.into()))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Keycode {
    Esc = 1,
    F1 = 2,
    F2 = 3,
    F3 = 4,
    F4 = 5,
    F5 = 6,
    F6 = 7,
    F7 = 8,
    F8 = 9,
    F9 = 0xA,
    F10 = 0xB,
    F11 = 0xC,
    F12 = 0xD,
    Home = 0xE,
    End = 0xF,
    Del = 0x10,
    Backquote = 0x15,
    Num1 = 0x16,
    Num2 = 0x17,
    Num3 = 0x18,
    Num4 = 0x19,
    Num5 = 0x1A,
    Num6 = 0x1B,
    Num7 = 0x1C,
    Num8 = 0x1D,
    Num9 = 0x1E,
    Num0 = 0x1F,
    Minus = 0x20,
    Equals = 0x21,
    Back = 0x24,
    Mute = 0x14,
    Tab = 0x29,
    Q = 0x2B,
    W = 0x2C,
    E = 0x2D,
    R = 0x2E,
    T = 0x2F,
    Y = 0x30,
    U = 0x31,
    I = 0x32,
    O = 0x33,
    P = 0x34,
    LeftBracket = 0x35,
    RightBracket = 0x36,
    Backslash = 0x38,
    AudioMute = 0x11,
    Caps = 0x3E,
    A = 0x3F,
    S = 0x40,
    D = 0x41,
    F = 0x42,
    G = 0x43,
    H = 0x44,
    J = 0x45,
    K = 0x46,
    L = 0x47,
    Semicolon = 0x48,
    Quote = 0x49,
    Enter = 0x4B,
    VolumeUp = 0x13,
    LShift = 0x52,
    Z = 0x54,
    X = 0x55,
    C = 0x56,
    V = 0x57,
    B = 0x58,
    N = 0x59,
    M = 0x5A,
    Comma = 0x5B,
    Period = 0x5C,
    Slash = 0x5D,
    RShift = 0x5F,
    Up = 0x73,
    VolumeDown = 0x12,
    LCtrl = 0x65,
    Fn = 0x66,
    LWin = 0x68,
    LAlt = 0x69,
    Space = 0x6A,
    RAlt = 0x70,
    RWin = 0x6E,
    RCtrl = 0x71,
    Left = 0x86,
    Down = 0x87,
    Right = 0x88,
}

impl Keycode {
    fn bytes(self, color: Color) -> Result<Vec<u8>> {
        let header = hex_to_bytes("cc8c0200")?;
        let a = ((self as u32) << 24 | u32::from(color)).to_be_bytes();
        let mut data = header;
        data.extend(a);
        data.resize(64, 0);
        Ok(data)
    }
}
