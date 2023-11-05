use crate::error::Result;
use rodio::cpal::traits::HostTrait;
use rodio::DeviceTrait;

/// Get a list of all available audio devices.
pub fn get_devices() -> Result<Vec<(String, rodio::Device)>> {
    Ok(rodio::cpal::default_host()
        .output_devices()?
        .map(|d| (d.name().unwrap_or("Unknown".to_string()), d))
        .collect())
}
