// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use ccs::*;
use gtk::prelude::*;

use crate::widgets::{InfoBox, InfoDescription};

type InitialParam = (String, String, gtk::SizeGroup);

component! {
    #[derive(Default)]
    pub struct InfoButton {}

    pub struct InfoButtonWidgets {}

    type Input = ();
    type Output = ();

    type Root = gtk::Box {
        InfoBox::default().0
    };

    fn init(args: InitialParam, root, input, output) {
        let (desc_label, button_label, sg) = args;

        ccs::view! {
            description = InfoDescription {
                set_label: &desc_label
            }
        }

        ccs::view! {
            button = gtk::Button {
                set_label: &button_label,

                connect_clicked(output) => move |_| {
                    let _ = output.send(());
                }
            }
        }

        root.append(description.widget());
        root.append(&button);

        sg.add_widget(&button);

        ComponentInner {
            model: InfoButton {},
            widgets: InfoButtonWidgets {},
            input,
            output
        }
    }

    fn update(_component, _message) {}
}
