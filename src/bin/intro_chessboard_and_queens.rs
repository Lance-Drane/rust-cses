// I/O boilerplate //

pub struct UnsafeScanner<'a> {
    // not actually dead code, needed for buf_iter to work
    #[allow(dead_code)]
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'a>,
}

impl UnsafeScanner<'_> {
    pub fn new<R: std::io::Read>(mut reader: R) -> Self {
        let mut buf_str = vec![];
        unsafe {
            reader.read_to_end(&mut buf_str).unwrap_unchecked();
        }
        let buf_iter = unsafe {
            let slice = std::str::from_utf8_unchecked(&buf_str);
            std::mem::transmute(slice.split_ascii_whitespace())
        };

        Self { buf_str, buf_iter }
    }

    /// Use "turbofish" syntax `token::<T>()` to select data type of next token.
    ///
    /// # Panics
    /// Panics if there's no more tokens or if the token cannot be parsed as T.
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        unsafe {
            self.buf_iter
                .next()
                .unwrap_unchecked()
                .parse()
                .unwrap_unchecked()
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
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
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

    writeln!(out, "{counter}").unwrap();
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
    let scan = UnsafeScanner::new(std::io::stdin());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    solve(scan, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(scan, &mut out);

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
