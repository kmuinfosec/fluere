use std::{fmt, time::Instant, borrow::Cow};

use crate::net::NetError;

use pcap::{Active, Address, Capture, Device, Error as PcapError};
use log::{info, debug};

#[derive(Debug)]
pub enum DeviceError {
    DeviceNotFound(String),
    InvalidDeviceIndex(usize),
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceError::DeviceNotFound(err) => write!(f, "Device not found: {}", err),
            DeviceError::InvalidDeviceIndex(err) => write!(f, "Invalid device index: {}", err),
        }
    }
}

pub struct CaptureDevice {
    pub name: Cow<'static, str>,
    pub desc: Cow<'static, str>,
    pub address: Vec<Address>,
    pub capture: Capture<Active>,
}

impl CaptureDevice {
    pub fn new(device: Device) -> Result<CaptureDevice, PcapError> {
        let capture = initialize_capture(device.clone())?;
        let name: Cow<'static, str> =  Cow::Owned(device.name);
        let desc: Cow<'static, str> = Cow::Owned(device.desc.unwrap_or("".to_string()));

        Ok(CaptureDevice {
            name,
            desc,
            address: device.addresses,
            capture,
        })
    }
}

impl Drop for CaptureDevice {
    fn drop(&mut self) {
        info!("Closing capture session for device {}", self.name);
        // println!("Closing capture session for device {}", self.name);
        // self.capture.;
    }
}
pub fn find_device(identifier: &str) -> Result<Device, NetError> {
    let start = Instant::now();
    debug!("Looking for device: {}", identifier);

    let devices = Device::list()?;

    if let Ok(index) = identifier.parse::<usize>() {
        if let Some(device) = devices.get(index) {
            let duration = start.elapsed();
            info!("Device {} captured in {:?}", device.name, duration);
            return Ok(device.clone());
        } else {
            return Err(NetError::DeviceError(DeviceError::InvalidDeviceIndex(
                index,
            )));
        }
    }

    for device in devices {
        if device.name == identifier {
            let duration = start.elapsed();
            info!("Device {} captured in {:?}", device.name, duration);
            return Ok(device);
        }
    }

    Err(NetError::DeviceError(DeviceError::DeviceNotFound(
        identifier.to_string(),
    )))
}

fn initialize_capture(device: Device) -> Result<Capture<Active>, PcapError> {
    info!("Opening capture session for device {}", device.name);
    Ok(Capture::from_device(device)?
        .promisc(true)
        .snaplen(1024)
        .timeout(60000)
        .immediate_mode(true)
        .open()?)
}
