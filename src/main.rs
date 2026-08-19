use std::io;

fn main() {
    loop {
        // Per the standard, unrecognized commands are simply ignored
        if let Some(command) = get_command() {
            match command {
                Command::UCI => uci(),
                Command::IsReady => is_ready()
            }
        }
    }
}

enum Command {
    UCI,
    IsReady
}

fn get_command() -> Option<Command> {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("failed to read line");

    match command.trim() {
        "uci" => Some(Command::UCI),
        "isready" => Some(Command::IsReady),
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

fn is_ready() {
    println!("readyok");
}
