use std::cell::{Cell, RefCell};
use std::mem::Discriminant;
use std::rc::{self, Rc};
use std::time::Duration;

use gtk::gdk::Display;
use gtk::gdk::keys::constants::Escape;
use gtk::glib::ffi::g_source_remove_by_user_data;
use gtk::glib::{self, Propagation, timeout_add_local};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box};
use gtk_layer_shell::LayerShell;

use crate::audio_panel::AudioPanel;
use crate::config::AppConfig;

pub struct MainWindow {
    container: Box,
    pub window: ApplicationWindow,
    pub audio_panel: AudioPanel,
}

impl MainWindow {
    pub fn new(widnow: ApplicationWindow) -> Self {
        let container = Box::new(gtk::Orientation::Vertical, 6);

        let audio_panel = AudioPanel::new();

        let self_ref = Self {
            container,
            window: widnow,
            audio_panel,
        };

        self_ref.init();

        self_ref
    }

    fn init(&self) {
        self.container.add(&self.audio_panel.container);
        self.window.add(&self.container);

        self.window.connect_key_press_event(|window, key| {
            if key.keyval() == Escape {
                window.close();
            }
            Propagation::Proceed
        });
    }

    pub fn show(&self) {
        self.window.show_all();
    }
}

pub trait CustomApplicationWindowBuilderImpl {
    fn build_from_config(config: &AppConfig, app: &Application) -> Result<MainWindow, ()>;
}

pub struct MainWindowBuilder {}

impl CustomApplicationWindowBuilderImpl for MainWindowBuilder {
    fn build_from_config(config: &AppConfig, app: &Application) -> Result<MainWindow, ()> {
        let window = ApplicationWindow::new(app);

        window.set_width_request(config.window.width);

        if let Some(height) = config.window.height {
            window.set_height_request(height);
        }

        window.init_layer_shell();
        window.set_layer(gtk_layer_shell::Layer::Top);
        window.set_anchor(gtk_layer_shell::Edge::Top, true);
        window.set_anchor(gtk_layer_shell::Edge::Right, true);
        window.auto_exclusive_zone_enable();
        window.set_keyboard_interactivity(true);

        if let Some(margin) = &config.window.margin {
            if let Some(left) = margin.left {
                window.set_layer_shell_margin(gtk_layer_shell::Edge::Left, left);
            }
            if let Some(top) = margin.top {
                window.set_layer_shell_margin(gtk_layer_shell::Edge::Top, top);
            }
            if let Some(right) = margin.right {
                window.set_layer_shell_margin(gtk_layer_shell::Edge::Right, right);
            }
            if let Some(buttom) = margin.buttom {
                window.set_layer_shell_margin(gtk_layer_shell::Edge::Bottom, buttom);
            }
        }

        Ok(MainWindow::new(window))
    }
}
