use std::io::{prelude::*, stderr, stdin, stdout, ErrorKind};

use rand::Rng;

/// Interactor for the solution, this is written more for stability and less for speed
///
/// Put all stderr output here
///
/// To use:
///   - mkfifo fifo
///   - cargo run --bin interactive_hidden_integer_solution <fifo | cargo run --bin interactive_hidden_integer_interactor >fifo
/// 
/// # Errors
///   raises `std::io::Error` if incorrect input format or IO somehow fails
///
/// # Panics
///   will not panic, `unwrap()` is called after we have verified the input string has at least one character
pub fn main() -> std::io::Result<()> {
    let mut buf = String::with_capacity(16);
    let mut std_in = stdin().lock();
    let mut std_out = stdout().lock();
    let mut std_err = stderr().lock();

    let ans = rand::thread_rng().gen_range(1..1_000_000_001);
    let mut queries = 0;

    writeln!(std_err, "Looking for {ans}")?;
    loop {
        std_in.read_line(&mut buf)?;
        let guess = buf
            .split_ascii_whitespace()
            .nth(1)
            .ok_or(ErrorKind::InvalidInput)?
            .parse::<u32>()
            .map_err(|_| ErrorKind::InvalidInput)?;
        match buf.chars().next().unwrap() {
            '?' => {
                queries += 1;
                if queries > 30 {
                    writeln!(std_err, "Too many queries")?;
                    std::process::exit(1);
                }
                writeln!(std_out, "{}", if guess < ans { "YES" } else { "NO" })?;
                writeln!(std_err, "Guessed {guess}")?;
                buf.clear();
            }
            '!' => {
                if guess == ans {
                    writeln!(std_err, "Guess correct")?;
                    std::process::exit(0);
                } else {
                    writeln!(std_err, "Guess incorrect")?;
                    std::process::exit(1);
                }
            }
            c => {
                writeln!(std_err, "Invalid operator '{c}'")?;
                std::process::exit(1);
            }
        }
    }
}
