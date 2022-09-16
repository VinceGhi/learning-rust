use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};

pub struct TerminalMenu {
    pub title: String,
    pub items: Vec<TerminalMenuItem>,
    pub selection_char: char,
}

impl TerminalMenu {
    pub fn select(&self) -> Option<fn()> {
        enter();
        let mut selection: u16 = 0;
        self.print_with_selection(&selection);
        loop {
            match crossterm::event::read() {
                Ok(event) => match event {
                    Event::FocusGained => {}
                    Event::FocusLost => {}
                    Event::Key(key_event) => {
                        let key: KeyCode = key_event.code;

                        if key == event::KeyCode::Esc {
                            exit();
                            return Option::None;
                        } else if key == event::KeyCode::Enter || key == event::KeyCode::Char(' ') {
                            let selected_menu_item: &TerminalMenuItem =
                                &self.items[selection as usize];
                            exit();
                            match selected_menu_item.action {
                                Some(action) => {
                                    return Option::Some(action);
                                }
                                None => {
                                    return Option::None;
                                }
                            }
                        } else if key == event::KeyCode::Up || key == event::KeyCode::Char('w') {
                            if selection == 0 {
                                selection = (self.items.len() - 1) as u16;
                            } else {
                                selection -= 1;
                            }
                        } else if key == event::KeyCode::Down || key == event::KeyCode::Char('s') {
                            selection += 1;
                            if selection >= self.items.len() as u16 {
                                selection = 0;
                            }
                        }

                        self.print_with_selection(&selection);
                    }
                    Event::Mouse(_) => {}
                    Event::Paste(_) => {}
                    Event::Resize(_, _) => {}
                },
                Err(_) => {}
            }
        }
    }

    fn print_with_selection(&self, selection: &u16) {
        move_cursor(0, 0);
        print!("{}", self.title);
        for i in 0..self.items.len() {
            move_cursor(0, 1 + (i as u16));
            if i == *selection as usize {
                print!("{} {}", self.selection_char, self.items[i].text);
            } else {
                print!("  {}", self.items[i].text);
            }
        }
        stdout().flush().unwrap();
    }
}

pub struct TerminalMenuItem {
    pub text: String,
    pub exit: bool,
    pub back: bool,
    pub action: Option<fn()>,
}

fn move_cursor(x: u16, y: u16) {
    stdout()
        .execute(MoveTo(x, y))
        .expect("Couldn't move Cursor!");
}

fn clear_terminal() {
    stdout()
        .execute(Clear(ClearType::All))
        .expect("Couldn't clear terminal!");
    move_cursor(0, 0);
}

fn enter() {
    crossterm::terminal::enable_raw_mode().expect("Rawmode can't be enabled?");
    stdout()
        .execute(crossterm::cursor::Hide)
        .expect("Couldn't hide cursor");
    clear_terminal();
}

fn exit() {
    crossterm::terminal::disable_raw_mode().expect("Rawmode can't be disabled?");
    stdout()
        .execute(crossterm::cursor::Show)
        .expect("Couldn't show cursor");
    clear_terminal();
}
