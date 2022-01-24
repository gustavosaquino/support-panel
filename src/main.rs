use cosmic_component_system::*;
use cosmic_support_panel::*;
use gtk4::prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = gresource::init();

    gtk::builders::ApplicationBuilder::new()
        .application_id("org.pop.Support")
        .cosmic_run(|app| {
            let window = gtk::ApplicationWindow::new(&app);

            window.set_child(Some(
                register(
                    crate::components::SupportPanel {
                        window: window.clone(),
                        vendor: None,
                    },
                    (),
                )
                .ignore()
                .widget(),
            ));

            window.show();
        });
}
