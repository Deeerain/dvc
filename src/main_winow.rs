use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box};
use gtk_layer_shell::LayerShell;

use crate::audio_panel::AudioPanel;

pub struct MainWindow {
    container: Box,
    pub window: ApplicationWindow,
    pub audio_panel: AudioPanel,
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
        self.window.set_width_request(250);
        self.window.set_height_request(75);
        self.window.init_layer_shell();
        self.window.set_layer(gtk_layer_shell::Layer::Top);
        self.window.set_anchor(gtk_layer_shell::Edge::Top, true);
        self.window.set_anchor(gtk_layer_shell::Edge::Right, true);

        self.window
            .set_layer_shell_margin(gtk_layer_shell::Edge::Top, 5);
        self.window
            .set_layer_shell_margin(gtk_layer_shell::Edge::Right, 5);

        self.container.add(&self.audio_panel.container);
        self.window.add(&self.container);
    }

    pub fn show(&self) {
        self.window.show_all();
    }
}
