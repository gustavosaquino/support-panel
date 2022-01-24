// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use ccs::*;
use gtk::prelude::*;

use crate::widgets::{InfoBox, InfoDescription};

component! {
    #[derive(Default)]
    pub struct InfoButton((String, String, gtk::SizeGroup)) {}

    pub struct InfoButtonWidgets(gtk::Box) {}

    type Input = ();
    type Output = ();

    fn init_view(self, args, _input, output) {
        let (desc_label, button_label, sg) = args;

        ccs::view! {
            description = InfoDescription {
                set_label: &desc_label
            }
        }

        ccs::view! {
            root = InfoBox {
                append: description.widget(),

                append: button = &gtk::Button {
                    set_label: &button_label,

                    connect_clicked(output) => move |_| {
                        let _ = output.send(());
                    }
                }
            }
        }

        sg.add_widget(&button);

        (InfoButtonWidgets {}, root.widget().clone())
    }

    fn update(self, _widgets, _message, _input, _output) {}
}
