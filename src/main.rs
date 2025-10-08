mod audio_panel;
mod auio;
mod main_winow;

use gtk::{
    Application,
    gio::{prelude::ApplicationExtManual, traits::ApplicationExt},
    glib::ExitCode,
    traits::RangeExt,
};
use std::error::Error;

use main_winow::*;

use crate::auio::AudioManager;

const APP_ID: &str = "com.github.deeerains.dvc";

fn main() -> Result<ExitCode, Box<dyn Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Logger inited");

    let app = Application::new(Some(APP_ID), Default::default());

    log::info!("Application inited");

    app.connect_activate(move |app| {
        let main_window = MainWindow::new(app);

        log::info!("Window inited");

        main_window
            .audio_panel
            .set_volume(AudioManager::get_volume().unwrap());

        main_window
            .audio_panel
            .volume_scale
            .connect_value_changed(move |scale| {
                AudioManager::set_volume(scale.value()).expect("Error");
            });

        main_window.show();
    });

    log::info!("Application running");

    let exit_code = app.run();

    Ok(exit_code)
}
