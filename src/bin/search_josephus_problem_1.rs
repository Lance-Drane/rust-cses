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

use std::collections::VecDeque;

/// Consider a game where there are n children (numbered 1,2,...,n) in a circle. During the game, every other child is removed from the circle until there are no children left. In which order will the children be removed?
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print n integers: the removal order.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let mut circle: VecDeque<u32> = (1..=scan.token()).collect();
    while circle.len() > 1 {
        circle.rotate_left(1);
        write!(out, "{} ", circle.pop_front().unwrap()).ok();
    }
    writeln!(out, "{}", circle.get(0).unwrap()).ok();
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
7
";
        let target = b"\
2 4 6 1 5 3 7
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
12
";
        let target = b"\
2 4 6 8 10 12 3 7 11 5 1 9
";

        test(input, target);
    }
}
