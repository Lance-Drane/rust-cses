// I/O boilerplate //

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: std::io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    /// Use "turbofish" syntax `token::<T>()` to select data type of next token.
    ///
    /// # Panics
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

// problem //

const COLUMNS: usize = 8;

/// Your task is to place eight queens on a chessboard so that no two queens are attacking each other. As an additional challenge, each square is either free or reserved, and you can only place queens on the free squares. However, the reserved squares do not prevent queens from attacking each other.
///
/// How many possible ways are there to place the queens?
///
/// <b>Input</b>
///
/// The input has eight lines, and each of them has eight characters. Each square is either free (.) or reserved (*).
///
/// <b>Output</b>
///
/// Print one integer: the number of ways you can place the queens.
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut chessboard = [0; COLUMNS * COLUMNS];
    for (idx, row) in (0..COLUMNS).map(|idx| (idx, scan.token::<String>().into_bytes())) {
        chessboard[COLUMNS * idx..COLUMNS * (idx + 1)].copy_from_slice(&row);
    }

    let mut visited_cols = [false; COLUMNS];
    let mut visited_diag_tl_br = [false; COLUMNS * 2 - 1];
    let mut visited_diag_tr_bl = [false; COLUMNS * 2 - 1];
    let mut counter = 0;

    recurse(
        0,
        &mut counter,
        &chessboard,
        &mut visited_cols,
        &mut visited_diag_tl_br,
        &mut visited_diag_tr_bl,
    );

    writeln!(out, "{counter}").ok();
}

fn recurse(
    row: usize,
    counter: &mut u8,
    chessboard: &[u8; COLUMNS * COLUMNS],
    visited_cols: &mut [bool; COLUMNS],
    visited_diag_tl_br: &mut [bool; COLUMNS * 2 - 1],
    visited_diag_tr_bl: &mut [bool; COLUMNS * 2 - 1],
) {
    if row == COLUMNS {
        *counter += 1;
        return;
    }
    for column in 0..COLUMNS {
        if visited_cols[column]
            || visited_diag_tl_br[row + column]
            || visited_diag_tr_bl[COLUMNS + column - row - 1]
            || chessboard[COLUMNS * row + column] == b'*'
        {
            continue;
        }

        visited_cols[column] = true;
        visited_diag_tl_br[row + column] = true;
        visited_diag_tr_bl[COLUMNS + column - row - 1] = true;

        recurse(
            row + 1,
            counter,
            chessboard,
            visited_cols,
            visited_diag_tl_br,
            visited_diag_tr_bl,
        );

        visited_cols[column] = false;
        visited_diag_tl_br[row + column] = false;
        visited_diag_tr_bl[COLUMNS + column - row - 1] = false;
    }
}

// entrypoints //

fn main() {
    let mut scan = UnsafeScanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    solve(&mut scan, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
........
........
..*.....
........
........
.....**.
...*....
........
";
        let target = b"\
65
";

        test(input, target);
    }
}
