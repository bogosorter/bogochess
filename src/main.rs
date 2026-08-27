use bogochess::uci::commands::{self, GUICommand};
use std::io;

fn main() {
    let mut position = None;

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("failed to read line");

        // Per the standard, unrecognized commands are simply ignored
        match commands::parse(&command) {
            Some(GUICommand::Quit) => break,
            Some(command) => if let Some(answer) = commands::process(command, &mut position) {
                println!("{answer}");
            },
            None => continue
        }
    }
}
