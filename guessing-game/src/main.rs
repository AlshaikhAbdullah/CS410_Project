extern crate gtk;
extern crate gio;
extern crate rand;

use gtk::prelude::*;
use gio::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ordering;
use rand::Rng;


#[derive(Debug)]
enum Mode {
	Normal,
	Hard
}

struct HeaderUi {
	headerbar: gtk::HeaderBar,
	switch: gtk::Switch,
	start_button: gtk::Button
}

impl HeaderUi {
	fn new() -> HeaderUi {
		let headerbar = gtk::HeaderBar::new();
		headerbar.set_title("Guessing Game");
		headerbar.set_show_close_button(true);

		let switch = gtk::Switch::new();
		let start_button = gtk::Button::new_with_label("Start");

		headerbar.pack_start(&switch);
		headerbar.pack_end(&start_button);

		HeaderUi { headerbar, switch, start_button }
	}
}

fn ui(app: &gtk::Application) {
	let secret: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(None));
	let mode: Rc<RefCell<Mode>> = Rc::new(RefCell::new(Mode::Normal));
	let window = gtk::ApplicationWindow::new(app);
	let header = HeaderUi::new();
	let switch = header.switch;
	let start_button = header.start_button;

	// Window HeaderBar
	window.set_titlebar(&header.headerbar);

	// (width, height);
	window.set_default_size(600, 300);

	// (orientation, spacing)
	let container = gtk::Box::new(
		gtk::Orientation::Vertical,
		0
	);
	container.set_sensitive(false);

	// Message
	let message_box = gtk::Box::new(
		gtk::Orientation::Horizontal,
		0
	);
	message_box.set_halign(gtk::Align::Center);
	let message = gtk::Label::new("Let's play");

	// (widget, expand, fill, padding)
	// Pack message
	message_box.pack_start(
		&message,
		false,
		false,
		0
	);

	// Entry
    let entry_box = gtk::box::new(
        gtk::Orientation::Horizontal,5
        );
    entry_box.set_halign(gtk::Align::Center);
    let entry = gtk::Entry::new();

    //pack entry
    entry_box.pack_start(
        &entry,
        false,
        false,
        0
    );
    //guess and stop buttons
    let guess_button = gtk::Button::new_with_label("Guess!");
    let stop_button = gtk::Button::new_with_label("Stop!");
    let button_box = gtk::Box::new(
        gtk::Orientation::Horizontal,
        5
    );
    button_box.pack_start(
        &guess_button,
        true,
        false,
        0
    );
    button_box.pack_start(
        &stop_button,
        true,
        false,
        0
    );

    // Add child box to container
    container.pack_start(
        &message_box,
        true,
        false,
        0
    );
    container.pack_start(
        &entry_box,
        true,
        false,
        0
    );
    container.pack_start(
        &button_box,
        true,
        false,
        0
    );




}

fn main() {
	let app = gtk::Application::new(
		"com.github.guessing_game",
		gio::ApplicationFlags::empty()
	).expect("Failed..");

	app.connect_startup(|app| {
		ui(&app);
	});

	//run app
	app.run(&std::env::args().collect::<Vec<_>>());
}
