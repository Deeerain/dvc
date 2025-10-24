use gtk::prelude::*;
use gtk::{Box, Scale};

pub struct AudioPanel {
    pub container: Box,
    volume_wrapper: Box,
    pub volume_scale: Scale,
}

impl AudioPanel {
    pub fn new() -> Self {
        let container = Box::new(gtk::Orientation::Vertical, 3);
        let volume_wrapper = Box::new(gtk::Orientation::Horizontal, 2);
        let volume_scale: Scale = Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 1.0);

        volume_scale.set_draw_value(false);

        let self_ref = Self {
            container,
            volume_wrapper,
            volume_scale,
        };

        self_ref.init();

        self_ref
    }

    fn init(&self) {
        self.volume_wrapper
            .pack_start(&self.volume_scale, true, true, 0);

        self.container
            .pack_start(&self.volume_wrapper, false, false, 0);
        self.container.set_margin_start(5);
        self.container.set_margin_end(5);
        self.container.set_margin_top(5);
        self.container.set_margin_bottom(5);
    }

    #[warn(dead_code)]
    pub fn set_volume(&self, value: f64) {
        self.volume_scale.set_value(value);
    }
}
