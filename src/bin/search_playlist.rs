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
            std::mem::transmute::<
                std::str::SplitAsciiWhitespace<'_>,
                std::str::SplitAsciiWhitespace<'_>,
            >(slice.split_ascii_whitespace())
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

use std::collections::HashMap;

/// You are given a playlist of a radio station since its establishment. The playlist has a total of n songs.
///
/// What is the longest sequence of successive songs where each song is unique?
///
/// <b>Input</b>
///
/// The first input line contains an integer n: the number of songs.
///
/// The next line has n integers k<sub>1</sub>,k<sub>2</sub<,...,k<sub<n</sub>: the id number of each song.
///
/// <b>Output</b>
///
/// Print the length of the longest sequence of unique songs.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ k<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let number: u32 = scan.token();

    let mut visited: HashMap<u32, u32> = HashMap::with_capacity(number as usize);
    let mut l_pointer = 0;
    let mut best = 0;

    for (song, idx) in (1..=number).map(|n| (scan.token(), n)) {
        if let Some(prev) = visited.insert(song, idx) {
            l_pointer = l_pointer.max(prev);
        }
        best = best.max(idx - l_pointer);
    }

    writeln!(out, "{best}").unwrap();
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
8
1 2 1 3 2 7 4 2
";
        let target = b"\
5
";

        test(input, target);
    }

    #[test]
    fn test_repetition() {
        let input = b"\
4
1000000000 1000000000 1000000000 1000000000
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_sample() {
        let input = b"\
20
4 1 1 4 8 9 7 6 5 9 4 9 7 3 10 3 8 3 9 6
";
        let target = b"\
7
";

        test(input, target);
    }
}
