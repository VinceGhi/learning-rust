use menu::{
    TerminalMenu, 
    TerminalMenuItem
};
use std::process;

mod menu;

fn main() {
    // this whole thing could somehow work i guess... but.. as someone mentioned on the official discord server for rust: i have to get rid of the java habbits :(
    create_main_menu();
}

fn create_main_menu() {
    let mut menu: TerminalMenu = TerminalMenu {title: "Test Menu".to_string(), items: Vec::new(), selection_char: '>'};
    menu.items = vec![
        TerminalMenuItem { text: "Sub Menu".to_string(), exit: false, action: Option::Some(create_sub_menu), back: false },
        TerminalMenuItem { text: "Entry two".to_string(), exit: false, action: Option::None, back: false },
        TerminalMenuItem { text: "Entry three".to_string(), exit: false, action: Option::None, back: false },
        TerminalMenuItem { text: "Entry four".to_string(), exit: false, action: Option::None, back: false },
        TerminalMenuItem { text: "Quit".to_string(), exit: true, action: Option::Some(exit), back: false }
    ];
    menu.select().expect("Well, there is no function...")();
}

fn create_sub_menu() {
    let mut sub_menu: TerminalMenu = TerminalMenu {title: "Test Sub Menu".to_string(), items: Vec::new(), selection_char: '>'};
    sub_menu.items= vec![
        TerminalMenuItem { text: "Back".to_string(), exit: false, action: Option::Some(create_main_menu), back: true },
        TerminalMenuItem { text: "Sub Sub Menu".to_string(), exit: false, action: Option::Some(create_sub_sub_menu), back: true },
        TerminalMenuItem { text: "Entry one".to_string(), exit: false, action: Option::None, back: false }
    ];
    sub_menu.select().expect("Well, there is no function...")();
}

fn create_sub_sub_menu() {
    let mut sub_menu: TerminalMenu = TerminalMenu {title: "Test Sub Sub Menu".to_string(), items: Vec::new(), selection_char: '>'};
    sub_menu.items= vec![
        TerminalMenuItem { text: "Back".to_string(), exit: false, action: Option::Some(create_sub_menu), back: true },
        TerminalMenuItem { text: "Entry one".to_string(), exit: false, action: Option::None, back: false }
    ];
    sub_menu.select().expect("Well, there is no function...")();
}

fn exit() {
    process::exit(1);
}
