use std::error::Error;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::modles::{AudioDevice, Margin, Side};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    pub width: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<Margin>,
    pub anchor: Option<Vec<Side>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub window: WindowConfig,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<AudioDevice>>,
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        if !path.exists() {
            let parent_folder = path.parent().unwrap();
            log::info!(
                "Config folder not exists: {0}",
                parent_folder.to_str().unwrap()
            );

            Self::create_default_config(path)?;
        }

        let config_content = fs::read_to_string(path.to_str().unwrap())?;
        let config = serde_json::from_str(&config_content)?;

        Ok(config)
    }

    pub fn create_default_config(path: &Path) -> Result<(), Box<dyn Error>> {
        let default_config = AppConfig::default();

        fs::create_dir_all(path.parent().unwrap())?;

        let config_json = serde_json::to_string_pretty(&default_config)?;
        fs::write(path, config_json)?;

        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                width: 300,
                height: None,
                margin: Some(Margin {
                    left: None,
                    top: Some(5),
                    right: Some(5),
                    buttom: Some(5),
                }),
                anchor: Some(Vec::from([Side::Right, Side::Top])),
            },
            devices: None,
        }
    }
}
