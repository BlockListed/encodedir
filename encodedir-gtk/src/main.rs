extern crate gtk4;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};

mod ui {
    pub mod filedialog;
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();
        
        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });

        let fchoose = ui::filedialog::new_filedialog(&window);

        fchoose.set_transient_for(Some(&window));

        window.set_child(Some(&button));

        window.show();

        fchoose.show();
    });

    application.run();
}