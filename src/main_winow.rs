use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box};

use crate::audio_panel::AudioPanel;

pub struct MainWindow {
    container: Box,
    window: ApplicationWindow,
    audio_panel: AudioPanel,
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        let container = Box::new(gtk::Orientation::Vertical, 6);
        let window = ApplicationWindow::builder().build();
        let audio_panel = AudioPanel::new();

        let self_ref = Self {
            container,
            window,
            audio_panel,
        };

        self_ref.init(app);

        self_ref
    }

    fn init(&self, app: &Application) {
        self.window.set_application(Some(app));
        self.container.add(&self.audio_panel.container);
        self.window.add(&self.container);
    }

    pub fn show(&self) {
        self.window.show_all();
    }
}
