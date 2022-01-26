use ccs::*;
use gtk::prelude::*;

pub enum LogEvent {
    Close,
    GeneratedLogs(anyhow::Result<String>),
    ShowInFolder,
}

component! {
    /// Manages the log generating dialog.
    pub struct LogDialog {
        pub dialog: Option<gtk::Dialog>,
        pub folder: Option<String>,
    }

    /// Widgets constructed by the view.
    pub struct LogWidgets {

    }

    type Input = LogEvent;
    type Output = ();

    type Root = gtk::Box {
        gtk::Box::default()
    };

    fn init(_args: (), root, input, output) {
        unimplemented!()
    }

    fn update(component, event) {
        unimplemented!()
    }
}
