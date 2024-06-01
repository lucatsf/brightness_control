use gtk::prelude::*;
use gtk::{Adjustment, Application, ApplicationWindow, ComboBoxText, Orientation, Scale};
use std::process::{Command};

fn main() {
    let application = Application::new(Some("com.lucatsf.brightness_control"), Default::default());

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Brightness Control");
        window.set_default_size(300, 100);

        let adjustment = Adjustment::new(50.0, 0.0, 100.0, 1.0, 10.0, 0.0);
        let scale = Scale::new(Orientation::Horizontal, Some(&adjustment));
        scale.set_digits(0);

        let monitors_combo = ComboBoxText::new();
        let monitors = list_monitors();
        for monitor in &monitors {
            monitors_combo.append_text(&monitor);
        }
        monitors_combo.set_active(Some(0));

        let monitors_combo_clone = monitors_combo.clone();
        scale.connect_value_changed(move |scale| {
            let value = scale.value() as i32;
            let monitor_index = monitors_combo_clone.active().unwrap();
            set_brightness(value, &monitors[monitor_index as usize]);
        });

        let vbox = gtk::Box::new(Orientation::Vertical, 0);
        vbox.pack_start(&scale, false, false, 0);
        vbox.pack_start(&monitors_combo, false, false, 0);

        window.add(&vbox);

        window.show_all();
    });

    application.run();
}

fn set_brightness(value: i32, monitor: &str) {
    let monitor_lowercase = monitor.to_lowercase();
    let command = format!("set{}", monitor_lowercase);
    let output = Command::new("sudo")
        .arg("ddcutil")
        .arg("--bus=3")
        .arg(command)
        .arg("10")
        .arg(value.to_string())
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Failed to set brightness: {:?}", output);
    }
}

fn list_monitors() -> Vec<String> {
    let output = Command::new("sudo")
        .arg("ddcutil")
        .arg("detect")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut monitors = vec![];

        for line in output_str.lines() {
            let line_lower = line.to_lowercase();
            if line_lower.contains("vcp version:") {
                let monitor_name = line.split_whitespace().next().unwrap_or("").to_string();
                monitors.push(monitor_name);
            }
        }

        monitors
    } else {
        eprintln!("Failed to list monitors: {:?}", output);
        Vec::new()
    }
}