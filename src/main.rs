mod audio_panel;
mod main_winow;

use std::error::Error;

use gtk::{
    Application,
    gio::{prelude::ApplicationExtManual, traits::ApplicationExt},
    glib::ExitCode,
};
use main_winow::*;

const APP_ID: &str = "com.github.deeerains.dvc";

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let app = Application::new(Some(APP_ID), Default::default());

    app.connect_activate(move |app| {
        let main_window = MainWindow::new(app);

        main_window.show();
    });

    let exit_code = app.run();

    Ok(exit_code)
}
