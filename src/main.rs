use cosmic_component_system::*;
use cosmic_support_panel::{components::SupportPanel, gresource};
use gtk4::prelude::*;

#[tokio::main]
async fn main() {
    let _ = gresource::init();

    gtk::builders::ApplicationBuilder::new()
        .application_id("org.pop.Support")
        .cosmic_run(|app| {
            let window = gtk::ApplicationWindow::new(&app);

            let panel = SupportPanel::init(window.clone()).ignore();

            window.set_child(Some(panel.widget()));

            window.show();
        });
}
