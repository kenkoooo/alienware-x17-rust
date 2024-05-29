#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("USB error: {0}")]
    Usb(#[from] rusb::Error),

    #[error("Invalid hex string: {0}")]
    InvalidHex(String),

    #[error("Device not found")]
    DeviceNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;
