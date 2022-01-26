use gtk::prelude::*;

#[derive(Clone)]
pub struct InfoBox(pub gtk::Box);

impl Default for InfoBox {
    fn default() -> Self {
        ccs::view! {
            container = gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_margin_start: 20,
                set_margin_end: 20,
                set_margin_top: 8,
                set_margin_bottom: 8,
                set_spacing: 24
            }
        }

        Self(container)
    }
}

impl ccs::Widget<gtk::Box> for InfoBox {
    fn widget(&self) -> &gtk::Box {
        &self.0
    }
}

impl std::ops::Deref for InfoBox {
    type Target = gtk::Box;

    fn deref(&self) -> &gtk::Box {
        &self.0
    }
}
