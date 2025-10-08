use alsa::{Mixer, mixer::SelemId};
use log::info;
use std::error::Error;

pub struct AudioManager;

impl AudioManager {
    pub fn get_volume() -> Result<f64, Box<dyn Error>> {
        let mixer = Mixer::new("default", false)?;
        let selem_id = SelemId::new("Master", 0);
        let elem = mixer.find_selem(&selem_id).ok_or("Can`t find elem")?;

        let (min, max) = elem.get_playback_volume_range();
        let volume = elem.get_playback_volume(alsa::mixer::SelemChannelId::FrontLeft)?;
        let volume_percent = (volume as f64 * 100.0) / max as f64;

        info!(
            "Get volume: max: {}, min: {}, volume: {}, percent: {}",
            max, min, volume, volume_percent
        );

        Ok(volume_percent)
    }

    pub fn set_volume(percent: f64) -> Result<(), alsa::Error> {
        let mixer = Mixer::new("default", false)?;
        let selem_id = SelemId::new("Master", 0);
        let elem = mixer.find_selem(&selem_id).ok_or("Err").unwrap();

        let (min, max) = elem.get_playback_volume_range();
        let volume = (max as f64 * percent) / 100.0;

        info!(
            "Set volume: min: {}, max: {}, volume: {}, percent: {}",
            min, max, volume, percent
        );

        elem.set_playback_volume_all(volume as i64)
    }
}
