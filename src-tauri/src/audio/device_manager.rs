use cpal::traits::{HostTrait, DeviceTrait};
use wasapi::{DeviceEnumerator, Direction};
use tracing::info;

pub struct DeviceManager;

impl DeviceManager {
    pub fn new() -> Self {
        Self
    }

    pub fn get_default_input_device(&self) -> Option<cpal::Device> {
        let host = cpal::default_host();
        host.default_input_device()
    }

    pub fn list_input_devices(&self) {
        let host = cpal::default_host();
        let devices = host.input_devices().unwrap();
        for device in devices {
            if let Ok(name) = device.name() {
                info!("Found Input Device: {}", name);
            }
        }
    }

    /// WASAPI Loopback requires a Render device to capture from.
    pub fn get_default_loopback_device(&self) -> anyhow::Result<wasapi::Device> {
        // We capture from the default render device in loopback mode
        let enumerator = DeviceEnumerator::new()?;
        let default_device = enumerator.get_default_device(&Direction::Render)?;
        if let Ok(name) = default_device.get_friendlyname() {
            info!("Default Loopback (Render) Device: {}", name);
        }
        Ok(default_device)
    }
}
