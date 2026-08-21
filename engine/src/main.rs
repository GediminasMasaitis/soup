mod search;
mod uci;

use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut input = String::new();

    while matches!(stdin.read_line(&mut input), Ok(n) if n > 0) {
        match input.trim() {
            "" => input.clear(),
            "uci" => {
                uci::run(stdin, std::io::stdout()).expect("IO error");
                return;
            }
            _ => return,
        }
    }
}
