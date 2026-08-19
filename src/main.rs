use std::io;

fn main() {
    loop {
        let parsed = get_command();
        if let Some(command) = parsed {
            match command {
                Command::UCI => uci()
            }
        } else {
            println!("unrecognized command");
        }
    }
}

enum Command {
    UCI
}

fn get_command() -> Option<Command> {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("failed to read line");

    match command.trim() {
        "uci" => Some(Command::UCI),
        _ => None
    }
}

fn uci() {
    let name = env!("CARGO_PKG_NAME");
    let author = env!("CARGO_PKG_AUTHORS");
    println!("id name {name}");
    println!("id author {author}");
    println!("uciok");
}
