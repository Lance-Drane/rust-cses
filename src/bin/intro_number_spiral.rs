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
        // optional memory clear
        buf_str.clear();

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
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
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
    let scan = UnsafeScanner::new(std::io::stdin().lock());
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
3
2 3
1 1
4 2
";
        let target = b"\
8
1
15
";

        test(input, target);
    }
}
