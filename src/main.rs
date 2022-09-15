use menu::{TerminalMenu, TerminalMenuItem};
use std::io::{stdout, Write, stdin, Read};

mod menu;

fn main() {
    // this whole thing could somehow work i guess... but.. as someone mentioned on the official discord server for rust: i have to get rid of the java habbits :(
    let mut sub_menu: TerminalMenu = TerminalMenu {title: "Test Menu".to_string(), items: Vec::new(), selection_char: '>'};
    let mut menu: TerminalMenu = TerminalMenu {title: "Test Menu".to_string(), items: Vec::new(), selection_char: '>'};
    let mut items: Vec<TerminalMenuItem> = Vec::new();
    items.push(TerminalMenuItem { text: "Entry one".to_string(), exit: false, menu: Option::None, back: false });
    items.push(TerminalMenuItem { text: "Back".to_string(), exit: false, menu: Option::None, back: true });
    sub_menu.items = items;

    let mut items: Vec<TerminalMenuItem> = Vec::new();
    items.push(TerminalMenuItem { text: "Entry one".to_string(), exit: false, menu: Option::Some(&sub_menu), back: false });
    items.push(TerminalMenuItem { text: "Entry two".to_string(), exit: false, menu: Option::Some(&sub_menu), back: false });
    items.push(TerminalMenuItem { text: "Entry three".to_string(), exit: false, menu: Option::None, back: false });
    items.push(TerminalMenuItem { text: "Entry four".to_string(), exit: false, menu: Option::None, back: false });
    items.push(TerminalMenuItem { text: "Quit".to_string(), exit: true, menu: Option::None, back: false });
    menu.items = items;

    loop {
        match menu.select() {
            Ok(selected_menu_item) => {
                println!("You've selected: {}", selected_menu_item.text);
                if selected_menu_item.exit {
                    break;
                }
                pause();
            },
            Err(_) => {
                break;
            }
        }
    }
}

fn pause() {
    let mut stdin = stdin();
    let mut stdout = stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}