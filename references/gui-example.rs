use gtk::{prelude::*, Entry, Label};
use gtk::{Application, ApplicationWindow, Button};
use gtk::Orientation::Vertical;

use std::fs;
use std::fs::File;

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    // How does `app` get passed to `build_ui()` here?
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK Example")
        .default_width(350)
        .default_height(70)
        .build();

    let vbox = gtk::Box::new(Vertical, 0);

    let lbl_source = Label::new(Some("Source path: "));
    vbox.add(&lbl_source);

    let txtbox_source = Entry::builder()
        .build();
    vbox.add(&txtbox_source);

    let lbl_dest = Label::new(Some("Destination path: "));
    vbox.add(&lbl_dest);

    let txtbox_dest = Entry::builder().build();
    vbox.add(&txtbox_dest);

    //let button = Button::with_label("Click me!");
    let button = Button::builder()
        .label("Move File")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to the "clicked" signal of `button`
    button.connect_clicked(move |_button| {
        // move keyword transfers ownership of variables into this closure function from the main
        // function. In this case it's the `txtbox_source` and `txtbox_dest` variables. 

        //button.set_label("Operation Successful!");

        // This is how Rust does Try/Catch. You can match (switch/case equivalent) a method that
        // returns a Result. But then you must handle both the `Ok` and `Err` variants
        // (Success/Error).
        match fs::rename(&txtbox_source.text(), &txtbox_dest.text()) {
            //Err(e) => { panic!("{:?}", e); }
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => match File::create(&txtbox_source.text()) {
                    Ok(fc) => fc,
                    Err(error) => panic!("Problem creating the file: {:?}", error),
                },
                other_error => {
                    panic!("Problem identifying error!");
                }
            },

            Ok(rename_output) => {
                eprintln!("File {} moved to {}", &txtbox_source.text(), &txtbox_dest.text());        
                println!("{:?}", rename_output);
            }
        }
    });

    vbox.add(&button);
    window.add(&vbox);

    // Show all widgets; without this the window won't show at all
    //window.present();

    /*
    * window.present() will draw the window without and widgets.
    * window.show_all() will draw the window and all widgets.
    */
    window.show_all();
}
