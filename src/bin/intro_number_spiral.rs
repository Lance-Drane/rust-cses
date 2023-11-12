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

/// A number spiral is an infinite grid whose upper-left square has number 1. Here are the first five layers of the spiral:
///
/// <pre>
/// 1  2  9  10 25
/// 4  3  8  11 24
/// 5  6  7  12 23
/// 16 15 14 13 22
/// 17 18 19 20 21
/// </pre>
///
/// Your task is to find out the number in row y and column x.
///
/// <b>Input</b>
///
/// The first input line contains an integer t: the number of tests.
///
/// After this, there are t lines, each containing integers y and x.
///
/// <b>Output</b>
///
/// For each test, print the number in row y and column x.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ t ≤ 10<sup>5</sup></li>
/// <li>1 ≤ y,x ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<u32>();

    for _ in 0..t {
        let y = scan.token::<u64>();
        let x = scan.token::<u64>();
        let max = x.max(y);

        writeln!(
            out,
            "{}",
            if max & 1 == 0 {
                // increment down and left
                max * max + 1 + y - x - max
            } else {
                // increment right and up
                max * max + 1 + x - y - max
            }
        )
        .ok();
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

    #[test]
    fn test_example() {
        let input: &[u8] = b"\
3
2 3
1 1
4 2
";
        let target: &[u8] = b"\
8
1
15
";

        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }
}
