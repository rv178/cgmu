extern crate num_cpus;

use nix::unistd::Uid;
use std::env::args;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;
use youchoose;

fn main() {
    for arg in args().skip(1) {
        let arg = &arg[..];
        match arg {
            "-h" | "--help" => {
                help();
            }
            "-m" | "--menu" => {
                perm_check();
                menu();
            }
            _ => {
                println!("Invalid option '{}'.", arg.to_string());
                exit(1);
            }
        }
    }
    perm_check();
    menu();
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
            println!("User chose 'performance'.");
        }
        1 => {
            if let Err(e) = push_prof("ondemand") {
                println!("An error occured! {}", e);
            }
            println!("User chose 'ondemand'.");
        }
        2 => {
            if let Err(e) = push_prof("schedutil") {
                println!("An error occured! {}", e);
            }
            println!("User chose 'schedutil'.");
        }
        3 => {
            if let Err(e) = push_prof("powersave") {
                println!("An error occured! {}", e);
            }
            println!("User chose 'powersave'.");
        }
        _ => println!("Choose a valid option!!"),
    }

    fn push_prof(cmd: &str) -> std::io::Result<()> {
        let cores = num_cpus::get();

        for core in 0..cores {
            let path = format!(
                "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor",
                core
            );
            let mut f = OpenOptions::new().write(true).truncate(true).open(path)?;
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

fn perm_check() {
    if Uid::current().is_root() {
        return;
    } else {
        println!("CGMU must be run as root! [Pass the -h flag to view the help message]");
        exit(1);
    }
}

fn help() {
    let help_msg = format!(
        "
CGMU - CPU governor management utility
[-h, --help] => show this message
[-m, --menu] => show an interactive menu
                           "
    );
    println!("{}", help_msg);
    exit(1);
}
