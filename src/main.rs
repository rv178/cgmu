extern crate num_cpus;

use youchoose;
use nix::unistd::Uid;
use std::fs;
use std::io::Write;

fn main() {
    if Uid::current().is_root() {
        menu();
    } else {
        println!("CGMU must be run as root!");
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

    match choice {
        0 => {
            if let Err(e) = push_prof("performance") {
                println!("An error occured! {}", e);
            }
            println!("Use chose 'performance'.");
        },
        1 => {
            if let Err(e) = push_prof("ondemand") {
                println!("An error occured! {}", e);
            }
            println!("Use chose 'ondemand'.");
        },
        2 => {
            if let Err(e) = push_prof("schedutil") {
                println!("An error occured! {}", e);
            }
            println!("Use chose 'schedutil'.");
        },
        3 => {
            if let Err(e) = push_prof("powersave") {
                println!("An error occured! {}", e);
            }
            println!("Use chose 'powersave'.");
        },
        _ => println!("Choose a valid option!!"),
    }

    fn push_prof(cmd: &str) -> std::io::Result<()> {
        let num = num_cpus::get();

        for n in 0..num {
            let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor", n);
            let mut f = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path)?;
            f.write_all(cmd.as_bytes())?;
            f.flush()?;
        }

        Ok(())
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
}
