use crossterm::{ExecutableCommand, terminal::{Clear, ClearType}, cursor::{MoveTo, SavePosition, RestorePosition, Hide, Show}};
use menu::{TerminalMenu};
use player::Player;
use core::time;
use std::{process, thread, io::stdout};

mod menu;
mod player;

fn main() {
    //progress_bar(time::Duration::from_secs(1), "Fake loading".to_string()); // i actually just wanted to test a little bit of stuff so... yeah... 
    let mut player: Player = Player::default();
    loop { // "game loop" damn.. i have no idea what i am doing :')
        start_menu(&mut player);
    }
}

fn start_menu(player: &mut Player) {
    let menu = TerminalMenu {
        title: "Test Menu".to_string(),
        entries: vec![
            "Select Class".to_string(), "Player Info".to_string(), "Quit".to_string(),
        ],
        selection_char: '>',
    };
    let selection = menu.select();

    if selection == 0 {
        select_class_menu(player);
    } else if selection == 1 {
        println!("{}", player.to_string());
        thread::sleep(time::Duration::from_secs(5));
    }else {
        process::exit(1);
    }
}

fn select_class_menu(player: &mut Player) {
    let class_selection_menu = TerminalMenu {
        title: "Class Selection".to_string(),
        entries: vec![
            "Class I".to_string(), "Class II".to_string(), "Class III".to_string()
        ],
        selection_char: '>',
    };

    let selection = class_selection_menu.select();

    if selection == 0 {
        player.set_stats("Class I".to_string(), 10, 0);
    } else if selection == 1 {
        player.set_stats("Class II".to_string(), 0, 10);
    } else if selection == 2 {
        player.set_stats("Class III".to_string(), 5, 5);
    }
}

//fn progress_bar(dur: time::Duration, text: String) {
//    let mut progress: u8 = 0;
//    stdout().execute(SavePosition).unwrap();
//    stdout().execute(Hide).unwrap();
//    while progress <= 100 {
//        if let Some((w, _)) = term_size::dimensions() {
//            let actually_bar_len = w - 3 - text.len();
//            let calc_progress: usize = (actually_bar_len as f64 / 100f64 * progress as f64).ceil() as usize;
//            let calc_empty: usize = actually_bar_len - calc_progress;
//            print!("{} [{}{}]", text, "■".repeat(calc_progress), "□".repeat(calc_empty));
//            stdout().execute(RestorePosition).unwrap();
//        }
//
//        thread::sleep(dur.div_f32(100f32));
//        progress += 1;
//    }
//    stdout().execute(Show).unwrap();
//    println!("");
//    thread::sleep(time::Duration::from_secs(1));
//}