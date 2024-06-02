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

/// Your task is to count for k=1,2,…,n the number of ways two knights can be placed on a k×k chessboard so that they do not attack each other.
///
/// <b>Input</b>
///
/// The only input line contains an integer n.
///
/// <b>Output</b>
///
/// Print n integers: the results.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10000</li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let token = scan.token::<u64>();

    for rows in 1..=token {
        let squares = rows * rows;

        // COMBINATIONS - KNIGHT_MOVES_POSSIBLE_ON_BOARD / 2 = ANSWER
        let combinations = squares * (squares - 1) / 2;
        // compute number of knight moves possible on board, i.e. the sum of the following grid:
        // 2 3 4 4 4 4 3 2
        // 3 4 6 6 6 6 4 3
        // 4 6 8 8 8 8 6 4
        // 4 6 8 8 8 8 6 4
        // 4 6 8 8 8 8 6 4
        // 4 6 8 8 8 8 6 4
        // 3 4 6 6 6 6 4 3
        // 2 3 4 4 4 4 3 2
        // (336)
        // ALL knight moves of square with N length = (n - 1) * (n - 2) * 8
        let all_knight_moves = (rows - 1) * (rows.wrapping_sub(2)) * 4; // allow underflow on k = 1
        let safe_knight_moves = combinations - all_knight_moves;
        writeln!(out, "{safe_knight_moves}").unwrap();
    }
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin());
    let mut out = std::io::BufWriter::with_capacity(32_768, std::io::stdout().lock());
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
8
";
        let target = b"\
0
6
28
96
252
550
1056
1848
";

        test(input, target);
    }
}
