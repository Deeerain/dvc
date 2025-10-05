use gtk::prelude::*;
use gtk::{Box, Button, ComboBoxText, Scale};

pub struct AudioPanel {
    pub container: Box,
    volume_wrapper: Box,
    volume_scale: Scale,
    device_cb: ComboBoxText,
    mute_button: Button,
}

impl AudioPanel {
    pub fn new() -> Self {
        let container = Box::new(gtk::Orientation::Vertical, 3);
        let volume_wrapper = Box::new(gtk::Orientation::Horizontal, 2);
        let mute_button = Button::with_label("m");
        let volume_scale: Scale = Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 1.0);

        volume_scale.set_draw_value(false);
        let device_cb = ComboBoxText::new();

        let self_ref = Self {
            container,
            volume_wrapper,
            volume_scale,
            device_cb,
            mute_button,
        };

        self_ref.init();

        self_ref
    }

    fn init(&self) {
        self.volume_wrapper
            .pack_start(&self.mute_button, false, false, 0);
        self.volume_wrapper
            .pack_start(&self.volume_scale, true, true, 0);

        self.container.pack_start(&self.device_cb, false, false, 0);
        self.container
            .pack_start(&self.volume_wrapper, false, false, 0);
    }

    #[warn(dead_code)]
    pub fn set_volume(&self, value: f64) {
        self.volume_scale.set_value(value);
    }

    pub fn get_volume(&self) -> f64 {
        self.volume_scale.value()
    }
}
