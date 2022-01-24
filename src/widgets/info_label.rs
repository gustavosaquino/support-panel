use super::{InfoBox, InfoDescription};
use ccs::Widget;
use gtk::prelude::*;

pub struct InfoLabel {
    root: gtk::Box,
    description: gtk::Label,
    value: gtk::Label,
}

impl InfoLabel {
    pub fn new(description: &str) -> Self {
        let info_label = InfoLabel::default();
        info_label.set_description(description);
        info_label
    }
}

impl Default for InfoLabel {
    fn default() -> Self {
        let description = InfoDescription::default();

        ccs::view! {
            root = InfoBox {
                append: description.widget(),

                append: value = &gtk::Label {
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::Center,
                }
            }
        }

        Self {
            root: root.widget().clone(),
            description: description.widget().clone(),
            value,
        }
    }
}

impl InfoLabel {
    pub fn set_description(&self, description: &str) {
        self.description.set_text(description);
    }

    pub fn set_value(&self, value: &str) {
        self.value.set_text(value);
    }
}

impl Widget<gtk::Box> for InfoLabel {
    fn widget(&self) -> &gtk::Box {
        &self.root
    }
}

impl std::ops::Deref for InfoLabel {
    type Target = gtk::Box;

    fn deref(&self) -> &gtk::Box {
        &self.root
    }
}
