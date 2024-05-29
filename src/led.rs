use rusb::UsbContext;

use crate::{device::AlienDeviceInner, Color, Result};

pub struct AlienLED<T: UsbContext> {
    device: AlienDeviceInner<T>,
}

impl<T: UsbContext> AlienLED<T> {
    pub fn new(context: &mut T) -> Result<AlienLED<T>> {
        Ok(AlienLED {
            device: AlienDeviceInner::new(context, 0x187C, 0x0550)?,
        })
    }

    fn write_packet(&self, packet: &[u8]) -> Result<()> {
        self.device.handle.write_control(
            0x21,
            0x09,
            0x0300,
            0x0000,
            packet,
            std::time::Duration::from_secs(5),
        )?;
        Ok(())
    }

    fn run_command(&self, fragment: Vec<u8>) -> Result<()> {
        let mut bytes = vec![0x03];
        bytes.extend(fragment);
        bytes.resize(33, 0);
        self.write_packet(&bytes)
    }

    pub fn change_color(&self, led: LED, color: Color) -> Result<()> {
        let command = hex_to_bytes("210001ffff")?;
        self.run_command(command)?;

        let command = start_series_command(1, &[led]);
        self.run_command(command)?;

        let command = add_action_command(&[Action {
            effect: ActionEffect::Color,
            duration: 2000,
            tempo: 250,
            color,
        }]);
        self.run_command(command)?;

        let command = hex_to_bytes("210003ffff")?;
        self.run_command(command)?;

        Ok(())
    }

    pub fn rainbow_wave(&self) -> Result<()> {
        let command = hex_to_bytes("210001ffff")?;
        self.run_command(command)?;

        let trons = [
            LED::Tron0,
            LED::Tron1,
            LED::Tron2,
            LED::Tron3,
            LED::Tron4,
            LED::Tron5,
            LED::Tron6,
            LED::Tron7,
            LED::Tron8,
            LED::Tron9,
        ];

        let rainbow_colors = [
            Color {
                red: 0x00,
                green: 0x00,
                blue: 0xff,
            },
            Color {
                red: 0x00,
                green: 0xa5,
                blue: 0xff,
            },
            Color {
                red: 0x00,
                green: 0xff,
                blue: 0xff,
            },
            Color {
                red: 0x00,
                green: 0x80,
                blue: 0x00,
            },
            Color {
                red: 0xff,
                green: 0xbf,
                blue: 0x00,
            },
            Color {
                red: 0xff,
                green: 0x00,
                blue: 0x00,
            },
            Color {
                red: 0x80,
                green: 0x00,
                blue: 0x80,
            },
        ];

        for (i, led) in trons.iter().enumerate() {
            let command = start_series_command(1, &[*led]);
            self.run_command(command)?;

            let colors1: Vec<_> = rainbow_colors.iter().cycle().skip(i).take(3).collect();
            let colors2: Vec<_> = rainbow_colors.iter().cycle().skip(i + 3).take(3).collect();
            let colors3: Vec<_> = rainbow_colors.iter().cycle().skip(i + 6).take(1).collect();

            for colors in [colors1, colors2, colors3] {
                let actions: Vec<_> = colors
                    .into_iter()
                    .map(|color| Action {
                        effect: ActionEffect::Morph,
                        duration: 0x01ac,
                        tempo: 0x000f,
                        color: *color,
                    })
                    .collect();
                let command = add_action_command(&actions);
                self.run_command(command)?;
            }
        }

        let command = hex_to_bytes("210003ffff")?;
        self.run_command(command)?;

        Ok(())
    }
}

fn start_series_command(loop_count: u8, zones: &[LED]) -> Vec<u8> {
    let mut fragment = vec![0x23];
    fragment.push(loop_count);
    fragment.extend_from_slice(&(zones.len() as u16).to_be_bytes());
    for zone in zones {
        fragment.push(*zone as u8);
    }
    fragment
}

fn add_action_command(actions: &[Action]) -> Vec<u8> {
    let mut fragment = vec![0x24];
    for action in actions.into_iter().take(3) {
        fragment.extend(action.bytes());
    }
    fragment
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
pub enum LED {
    PowerButton = 0,
    Head = 1,
    Tron0 = 10,
    Tron1 = 11,
    Tron2 = 12,
    Tron3 = 13,
    Tron4 = 14,
    Tron5 = 15,
    Tron6 = 16,
    Tron7 = 17,
    Tron8 = 18,
    Tron9 = 19,
}

#[derive(Debug, Clone, Copy)]
pub enum ActionEffect {
    Color = 0x00,
    Pulse = 0x01,
    Morph = 0x02,
}

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub effect: ActionEffect,
    pub duration: u16,
    pub tempo: u16,
    pub color: Color,
}

impl Action {
    fn bytes(self) -> Vec<u8> {
        let mut bytes = vec![self.effect as u8];
        bytes.extend(self.duration.to_be_bytes());
        bytes.extend(self.tempo.to_be_bytes());
        bytes.push(self.color.red);
        bytes.push(self.color.green);
        bytes.push(self.color.blue);
        bytes
    }
}
