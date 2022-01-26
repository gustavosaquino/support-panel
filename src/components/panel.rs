// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::components::InfoButton;
use crate::support_info::SupportInfo;
use crate::vendor::Vendor;
use crate::widgets::{InfoBox, InfoLabel};
use ccs::*;
use gtk::prelude::*;

pub enum SupportEvent {
    BrowseDocumentation,
    CommunitySupport,
    CreateLogFiles,
    CreateSupportTicket,
    LoadInfo,
    UpdateInfo(SupportInfo),
}

pub enum SupportCmd {
    LoadInfo,
}

elm_component! {
    pub struct SupportPanel {
        pub window: gtk::ApplicationWindow,
        pub info: Option<SupportInfo>,
    }

    pub struct SupportWidgets {
        image: gtk::Image,
        model_info: InfoLabel,
        serial_info: InfoLabel,
        os_info: InfoLabel,
        professional: Handle<gtk::Box, ()>,
    }

    type Input = SupportEvent;
    type Output = ();
    type Command = SupportCmd;

    type Root = gtk::ScrolledWindow {
        gtk::builders::ScrolledWindowBuilder::new()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build()
    };

    fn init(window: gtk::ApplicationWindow, root, input, output) {
        // Begin loading info required by this page in the background.
        let _ = input.send(SupportEvent::LoadInfo);

        let button_sg = gtk::SizeGroup::new(gtk::SizeGroupMode::Both);
        let list_sg = gtk::SizeGroup::new(gtk::SizeGroupMode::Both);

        let model_info = InfoLabel::new(&crate::fl!("model-and-version"));
        let serial_info = InfoLabel::new(&crate::fl!("serial-number"));
        let os_info = InfoLabel::new(&crate::fl!("os-version"));

        let documentation = InfoButton::init((
                crate::fl!("documentation"),
                crate::fl!("documentation-button"),
                button_sg.clone(),
            ))
            .forward(input.clone(), |_| SupportEvent::BrowseDocumentation);

        let community = InfoButton::init((
                crate::fl!("support-community"),
                crate::fl!("support-community-button"),
                button_sg.clone(),
            ))
            .forward(input.clone(), |_| SupportEvent::CommunitySupport);

        let professional = InfoButton::init((
                crate::fl!("support-professional"),
                crate::fl!("support-professional-button"),
                button_sg.clone(),
            ))
            .forward(input.clone(), |_| SupportEvent::CreateSupportTicket);

        let log_generate = InfoButton::init((
                crate::fl!("create-logs"),
                crate::fl!("create-logs-button"),
                button_sg,
            ))
            .forward(input.clone(), |_| SupportEvent::CreateLogFiles);

        ccs::view! {
            gtk::SizeGroup::new(gtk::SizeGroupMode::Both) {
                add_widget: &*model_info,
                add_widget: &*serial_info,
                add_widget: &*os_info,
                add_widget: &*documentation.widget(),
                add_widget: &*community.widget(),
                add_widget: &*professional.widget(),
                add_widget: &*log_generate.widget(),
            }
        };

        ccs::view! {
            inner = gtk::Box {
                set_halign: gtk::Align::Center,
                set_orientation: gtk::Orientation::Vertical,

                append: image = &gtk::Image {
                    set_margin_top: 24,
                    set_margin_bottom: 24,
                    set_size_request: args!(256, 256),
                },

                append = &gtk::ListBox {
                    set_header_func: separator_header,
                    set_selection_mode: gtk::SelectionMode::None,
                    append: &*model_info,
                    append: &*serial_info,
                    append: &*os_info,
                    append: &*documentation.widget(),
                    append: &*community.widget(),
                    append: &*professional.widget(),
                    append: &*log_generate.widget(),
                }
            }
        }

        root.set_child(Some(&inner));

        ComponentInner {
            model: SupportPanel {
                window,
                info: None
            },
            widgets: SupportWidgets {
                image,
                model_info,
                serial_info,
                os_info,
                professional,
            },
            input,
            output
        }
    }

    fn update(self, event, _input, _output) {
        match event {
            SupportEvent::LoadInfo => {
                eprintln!("got LoadInfo");
                return Some(SupportCmd::LoadInfo);
            }
            SupportEvent::UpdateInfo(info) => {
                self.info = Some(info);
            }

            SupportEvent::BrowseDocumentation => {
                open_url("https://support.system76.com");
            }

            SupportEvent::CommunitySupport => open_url("https://chat.pop-os.org"),

            SupportEvent::CreateSupportTicket => {
                open_url("https://system76.com/my-account/support-tickets/new")
            }

            SupportEvent::CreateLogFiles => {
                // #[allow(clippy::single_match)]
                // match self.vendor {
                //     None | Some(Vendor::System76) => {
                //         let dialog = gtk::DialogBuilder::new()
                //             .transient_for(&self.model.window)
                //             .modal(true)
                //             .decorated(false)
                //             .resizable(false)
                //             .default_width(480)
                //             .build();

                //         let dialog_inner = relm::init::<LogDialog>(dialog.clone()).unwrap();

                //         dialog.content_area().add(dialog_inner.widget());

                //         dialog.show();

                //         let stream = dialog_inner.stream();
                //         let (_channel, sender) = relm::Channel::new(move |result| {
                //             stream.emit(LogEvent::GeneratedLogs(result))
                //         });

                //         std::thread::spawn(move || {
                //             let _ = sender.send(generate_logs_subprocess());
                //         });zm

                //         // Keeps the event stream alive for as long as the dialog needs it.
                //         self.model.log_dialog = Some(dialog_inner);
                //     }
                // }
            }
        }

        None
    }

    fn update_view(self, widgets, input, output) {
        if let Some(info) = self.info.as_ref() {
            // Update the info labels with the fetched info.
            widgets.model_info.set_value(&info.model_and_version);
            widgets.os_info.set_value(&info.operating_system);

            // Hide the serial info widget if there is no serial number.
            if let Some(parent) = widgets.serial_info.parent() {
                if info.serial_number.is_empty() {
                    parent.hide();
                } else {
                    parent.show();
                    widgets.serial_info.set_value(&info.serial_number);
                }
            }

            // Function for updating the panel image using a gresource.
            let set_by_resource = |resource: &str| {
                let pixbuf =
                    gdk_pixbuf::Pixbuf::from_resource_at_scale(resource, 256, 256, true);

                if let Ok(pixbuf) = pixbuf {
                    widgets.image.set_from_pixbuf(Some(&pixbuf));
                }
            };

            // If a vendor is found, set the image for that that vendor.
            // Also conditionally show the professional info button for that vendor.
            if let Some(parent) = widgets.professional.widget().parent() {
                if let Some(vendor) = info.vendor {
                    parent.show();

                    match vendor {
                        Vendor::System76 => set_by_resource("/org/pop/support/system76.png"),
                    }
                } else {
                    parent.hide();

                    widgets
                        .image
                        .set_from_icon_name(Some("distributor-logo-pop-os"));
                }
            }
        }
    }

    async fn command(event) {
        if let SupportCmd::LoadInfo = event {
            eprintln!("fetching support info");
            let mut info = SupportInfo::fetch().await;

            if info.model_and_version.is_empty() {
                info.model_and_version = crate::fl!("unknown")
            }

            if info.operating_system.is_empty() {
                info.operating_system = crate::fl!("unknown");
            }

            if info.serial_number.is_empty() {
                info.serial_number = crate::fl!("unknown");
            }

            Some(SupportEvent::UpdateInfo(info))
        } else {
            None
        }
    }
}

fn open_url(url: &'static str) {
    std::thread::spawn(move || {
        let _ = std::process::Command::new("xdg-open").arg(url).status();
    });
}

fn separator_header(current: &gtk::ListBoxRow, before: Option<&gtk::ListBoxRow>) {
    if before.is_some() {
        current.set_header(Some(&gtk::Separator::new(gtk::Orientation::Horizontal)));
    }
}
