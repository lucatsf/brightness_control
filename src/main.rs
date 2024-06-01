use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Scale, Orientation, Adjustment};
use std::process::Command;

fn main() {
    let application = Application::new(
        Some("com.example.brightness_control"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Brightness Control");
        window.set_default_size(300, 70);

        let adjustment = Adjustment::new(50.0, 0.0, 100.0, 1.0, 10.0, 0.0);
        let scale = Scale::new(Orientation::Horizontal, Some(&adjustment));

        scale.set_digits(0);

        scale.connect_value_changed(|scale| {
            let value = scale.value() as i32;
            set_brightness(value);
        });

        window.add(&scale);
        window.show_all();
    });

    application.run();
}

fn set_brightness(value: i32) {
    let output = Command::new("sudo")
        .arg("ddcutil")
        .arg("--bus=3")
        .arg("setvcp")
        .arg("10")
        .arg(value.to_string())
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Failed to set brightness: {:?}", output);
    }
}
