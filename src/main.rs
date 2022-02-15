//use std::process::Command;
use youchoose;

fn main() {
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

    if choice == 0 {
        println!("You chose 1");
    } else if choice == 1 {
        println!("You chose 2");
    } else if choice == 2 {
        println!("You chose 3");
    } else if choice == 3 {
        println!("You chose 4");
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
