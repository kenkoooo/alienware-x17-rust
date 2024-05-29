use rusb::{DeviceHandle, UsbContext};

use crate::Result;

pub(crate) struct AlienDeviceInner<T: UsbContext> {
    pub(crate) handle: DeviceHandle<T>,
}

impl<T: UsbContext> AlienDeviceInner<T> {
    pub(crate) fn new(context: &mut T, vid: u16, pid: u16) -> Result<AlienDeviceInner<T>> {
        let handle = open_device(context, vid, pid)?;
        if handle.kernel_driver_active(0)? {
            handle.detach_kernel_driver(0)?;
        }

        handle.set_auto_detach_kernel_driver(true)?;
        handle.set_active_configuration(1)?;
        handle.claim_interface(0)?;
        Ok(AlienDeviceInner { handle })
    }
}

fn open_device<T: UsbContext>(context: &mut T, vid: u16, pid: u16) -> Result<DeviceHandle<T>> {
    let devices = context.devices()?;

    for device in devices.iter() {
        let device_desc = device.device_descriptor()?;

        if device_desc.vendor_id() != vid || device_desc.product_id() != pid {
            continue;
        }

        let handle = device.open()?;
        return Ok(handle);
    }

    Err(crate::Error::DeviceNotFound)
}
