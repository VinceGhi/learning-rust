use crossterm::{ExecutableCommand, terminal::{Clear, ClearType}, cursor::MoveTo};
use menu::{TerminalMenu, TerminalMenuItem};
use core::time;
use std::{process, thread, io::stdout};

mod menu;

fn main() {
    // this whole thing could somehow work i guess... but.. as someone mentioned on the official discord server for rust: i have to get rid of the java habbits :(
    create_main_menu();
}

fn create_main_menu() {
    let mut menu: TerminalMenu = TerminalMenu {
        title: "Test Menu".to_string(),
        items: Vec::new(),
        selection_char: '>',
    };
    menu.items = vec![
        TerminalMenuItem {
            text: "Sub Menu".to_string(),
            exit: false,
            action: Option::Some(create_sub_menu),
            back: false,
        },
        TerminalMenuItem {
            text: "Progressbar".to_string(),
            exit: false,
            action: Option::Some(progress_bar),
            back: false,
        },
        TerminalMenuItem {
            text: "No fn".to_string(),
            exit: false,
            action: Option::None,
            back: false,
        },
        TerminalMenuItem {
            text: "No fn".to_string(),
            exit: false,
            action: Option::None,
            back: false,
        },
        TerminalMenuItem {
            text: "Quit".to_string(),
            exit: true,
            action: Option::Some(exit),
            back: false,
        },
    ];
    menu.select().expect("Well, there is no function...")();
}

fn create_sub_menu() {
    let mut sub_menu: TerminalMenu = TerminalMenu {
        title: "Test Sub Menu".to_string(),
        items: Vec::new(),
        selection_char: '>',
    };
    sub_menu.items = vec![
        TerminalMenuItem {
            text: "Back".to_string(),
            exit: false,
            action: Option::Some(create_main_menu),
            back: true,
        },
        TerminalMenuItem {
            text: "Progressbar".to_string(),
            exit: false,
            action: Option::Some(progress_bar),
            back: false,
        },
        TerminalMenuItem {
            text: "Sub Sub Menu".to_string(),
            exit: false,
            action: Option::Some(create_sub_sub_menu),
            back: true,
        },
        TerminalMenuItem {
            text: "Entry one".to_string(),
            exit: false,
            action: Option::None,
            back: false,
        },
    ];
    sub_menu.select().expect("Well, there is no function...")();
}

fn create_sub_sub_menu() {
    let mut sub_menu: TerminalMenu = TerminalMenu {
        title: "Test Sub Sub Menu".to_string(),
        items: Vec::new(),
        selection_char: '>',
    };
    sub_menu.items = vec![
        TerminalMenuItem {
            text: "Back".to_string(),
            exit: false,
            action: Option::Some(create_sub_menu),
            back: true,
        },
        TerminalMenuItem {
            text: "Progressbar".to_string(),
            exit: false,
            action: Option::Some(progress_bar),
            back: false,
        },
        TerminalMenuItem {
            text: "Entry one".to_string(),
            exit: false,
            action: Option::None,
            back: false,
        },
    ];
    sub_menu.select().expect("Well, there is no function...")();
}

fn exit() {
    process::exit(1);
}

fn progress_bar() {
    let mut progress: u8 = 0;
    while progress <= 100 {
        stdout().execute(Clear(ClearType::All)).unwrap();

        if let Some((w, _)) = term_size::dimensions() {
            let calc_progress: usize = ((w.min(100) - 3) as f64 / 100f64 * progress as f64).ceil() as usize;
            let calc_empty: usize = w.min(100) - 3 - calc_progress;
            print!("[{}{}]","#".repeat(calc_progress), "-".repeat(calc_empty));
            stdout().execute(MoveTo(0, 0)).unwrap();
        }

        thread::sleep(time::Duration::from_millis(50));
        progress += 1;
    }
    println!("\nDone with fake loading thing...");
    thread::sleep(time::Duration::from_secs(5));
    create_main_menu();
}