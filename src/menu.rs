use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    terminal::{Clear, ClearType},
    ExecutableCommand, style::{Stylize, StyledContent},
};
use std::io::{stdout, Write};

pub struct TerminalMenu {
    pub title: String,
    pub entries: Vec<String>,
    pub selection_char: char
}

impl TerminalMenu {
    pub fn select(&self) -> u8 {
        enter();
        let mut selection: i8 = 0;
        self.print_with_selection(&selection);
        loop {
            let event = crossterm::event::read().expect("Not good :(");

            if let Event::Key(key_event) = event {
                let key: KeyCode = key_event.code;
                if key == event::KeyCode::Enter || key == event::KeyCode::Char(' ') {
                    exit();
                    return selection as u8;
                } else if key == event::KeyCode::Up || key == event::KeyCode::Char('w') {
                    selection = self.change_selection(selection, -1);
                } else if key == event::KeyCode::Down || key == event::KeyCode::Char('s') {
                    selection = self.change_selection(selection, 1);
                }

                self.print_with_selection(&selection);
            } else if let Event::Resize(_, _) = event {
                self.print_with_selection(&selection);
            }
        }
    }

    fn print_with_selection(&self, selection: &i8) {
        move_cursor(0, 0);
        print!("{}", self.title);
        for i in 0..self.entries.len() {
            move_cursor(0, 1 + (i as u16));

            let is_current_selection: bool = i == *selection as usize;

            let item_text: StyledContent<String> = if is_current_selection {
                format!("{}", self.entries[i]).blue().bold()
            } else {
                format!("{}", self.entries[i]).reset()
            };

            if is_current_selection {
                print!("{} {}", self.selection_char, item_text);
            } else {
                print!("  {}", item_text);
            }
        }
        stdout().flush().unwrap();
    }

    fn change_selection(&self, selection: i8, change: i8) -> i8 {
        let mut new_selection: i8 = selection + change;

        if new_selection < 0 {
            new_selection = (self.entries.len() as i8) - 1;
        } else if new_selection >= self.entries.len() as i8 {
            new_selection = 0;
        }

        return new_selection;
    }
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