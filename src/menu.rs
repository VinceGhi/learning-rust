use core::time;
use std::io::{stdout, Write};
use crossterm::{cursor::MoveTo, ExecutableCommand, terminal::{ClearType, Clear}, event::{Event, self, KeyCode}};

pub struct TerminalMenu<'a> {
    pub title: String,
    pub items: Vec<TerminalMenuItem<'a>>,
    pub selection_char: char
}

impl TerminalMenu<'_> {
    pub fn select(&self) -> Result<&TerminalMenuItem, String> {
        enter();
        let mut selection: u16 = 0;
        self.print_with_selection(&selection);
        loop {
           let event_is_waiting: bool = crossterm::event::poll(time::Duration::from_millis(20)).expect("Coudln't check if a terminal event is there...");

           if event_is_waiting {
            match crossterm::event::read() {
                Ok(event) => {
                    match event {
                        Event::FocusGained => {},
                        Event::FocusLost => {},
                        Event::Key(key_event) => {
                            let key: KeyCode = key_event.code;

                            if key == event::KeyCode::Esc {
                                exit();
                                return Err("User pressed ESC".to_string());
                            } else if key == event::KeyCode::Enter || key == event::KeyCode::Char(' ') {
                                exit();
                                let selected_menu_item: &TerminalMenuItem = &self.items[selection as usize];
                                if selected_menu_item.menu.is_some() {
                                    let item: &TerminalMenuItem = selected_menu_item.menu.unwrap().select().expect("Something went wrong!");
                                    if item.back {
                                        return Ok(item);
                                    } else {
                                        return item.menu.expect("Woopsie!").select();
                                    }
                                }
                                
                                return Ok(selected_menu_item);
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
                        },
                        Event::Mouse(_) => {},
                        Event::Paste(_) => {},
                        Event::Resize(_, _) => {}
                    }
                },
                Err(_) => {},
            }
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

pub struct TerminalMenuItem<'a> {
    pub text: String,
    pub exit: bool,
    pub back: bool,
    pub menu: Option<&'a TerminalMenu<'a>>
}

fn move_cursor(x: u16, y: u16) {
    stdout().execute(MoveTo(x, y)).expect("Couldn't move Cursor!");
}

fn clear_terminal() {
    stdout().execute(Clear(ClearType::All)).expect("Couldn't clear terminal!");
    move_cursor(0, 0);
}

fn enter() {
    clear_terminal();
    stdout().execute(crossterm::cursor::Hide).expect("Couldn't hide cursor");
    crossterm::terminal::enable_raw_mode().expect("Rawmode can't be enabled?");
}

fn exit() {
    clear_terminal();
    stdout().execute(crossterm::cursor::Show).expect("Couldn't show cursor");
    crossterm::terminal::disable_raw_mode().expect("Rawmode can't be disabled?");
}