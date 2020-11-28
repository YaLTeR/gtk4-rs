//! # Entry: Enabling undo
use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Entry Undo");
    window.set_default_size(450, 250);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 12);
    vbox.set_halign(gtk::Align::Center);
    vbox.set_valign(gtk::Align::Center);

    let label = gtk::Label::new(Some(
        "Use Control + Z or Control + Shift + Z to redo changes",
    ));
    vbox.append(&label);

    let entry = gtk::Entry::new();
    entry.set_enable_undo(true);
    vbox.append(&entry);

    window.set_child(Some(&vbox));

    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.entry-undo"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
