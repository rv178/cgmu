use nix::unistd::Uid;
use std::process::Command;
use youchoose;

fn main() {
    if !Uid::current().is_root() {
        println!("CGMU must be run as root!");
    } else {
        menu();
    }
}

fn menu() {
    let mut menu = youchoose::Menu::new(0..4)
        .preview(preview_menu)
        .preview_pos(youchoose::ScreenSide::Bottom, 0.4)
        .preview_label("Choice Preview".to_string())
        .icon("->");
    let choice = menu.show();
    let choice: usize = choice[0];

    if choice == 0 {
        let _output = Command::new("sh")
            .arg("-c")
            .arg("echo 'performance' | tee /sys/devices/system/cpu/*/cpufreq/scaling_governor")
            .output()
            .expect("Failed to execute command");
        println!("User chose 'performance'.");
    } else if choice == 1 {
        let _output = Command::new("sh")
            .arg("-c")
            .arg("echo 'ondemand' | tee /sys/devices/system/cpu/*/cpufreq/scaling_governor")
            .output()
            .expect("Failed to execute command");
        println!("User chose 'ondemand'.");
    } else if choice == 2 {
        let _output = Command::new("sh")
            .arg("-c")
            .arg("echo 'schedutil' | tee /sys/devices/system/cpu/*/cpufreq/scaling_governor")
            .output()
            .expect("Failed to execute command");
        println!("User chose 'schedutil'.");
    } else if choice == 3 {
        let _output = Command::new("sh")
            .arg("-c")
            .arg("echo 'powersave' | tee /sys/devices/system/cpu/*/cpufreq/scaling_governor")
            .output()
            .expect("Failed to execute command");
        println!("User chose 'powersave'.");
    } else {
        println!("Your choice was {}.", choice);
    }
}

fn preview_menu(num: i32) -> String {
    let mut buffer = String::new();
    if num == 0 {
        buffer.push_str(&format!("Performance\n"));
    } else if num == 1 {
        buffer.push_str(&format!("Ondemand\n"));
    } else if num == 2 {
        buffer.push_str(&format!("Schedutil\n"));
    } else if num == 3 {
        buffer.push_str(&format!("Powersave\n"));
    }
    buffer
}
