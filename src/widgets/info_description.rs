use gtk::prelude::*;
pub struct InfoDescription(gtk::Label);

impl Default for InfoDescription {
    fn default() -> Self {
        ccs::view! {
            label = gtk::Label {
                set_halign: gtk::Align::Start,
                set_hexpand: true,
                set_valign: gtk::Align::Center,
                set_ellipsize: gtk::pango::EllipsizeMode::End,
            }
        }

        Self(label)
    }
}

impl ccs::Widget<gtk::Label> for InfoDescription {
    fn widget(&self) -> &gtk::Label {
        &self.0
    }
}

impl std::ops::Deref for InfoDescription {
    type Target = gtk::Label;

    fn deref(&self) -> &gtk::Label {
        &self.0
    }
}
