use ccs::*;
use gtk::prelude::*;

pub enum LogEvent {
    Close,
    GeneratedLogs(anyhow::Result<String>),
    ShowInFolder,
}

component! {
    /// Manages the log generating dialog.
    pub struct LogDialog(()) {
        pub dialog: Option<gtk::Dialog>,
        pub folder: Option<String>,
    }

    /// Widgets constructed by the view.
    pub struct LogWidgets(gtk::Box) {

    }

    type Input = LogEvent;
    type Output = ();

    fn init_view(self, args, input, output) {
        unimplemented!()
    }

    fn update(self, widgets, event, input, output) {
        unimplemented!()
    }
}
