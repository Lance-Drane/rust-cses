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

/// You have two coin piles containing a and b coins. On each move, you can either remove one coin from the left pile and two coins from the right pile, or two coins from the left pile and one coin from the right pile.
///
/// Your task is to efficiently find out if you can empty both the piles.
///
/// <b>Input</b>
///
/// The first input line has an integer t: the number of tests.
///
/// After this, there are t lines, each of which has two integers a and b: the numbers of coins in the piles.
///
/// <b>Output</b>
///
/// For each test, print "YES" if you can empty the piles and "NO" otherwise.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ t ≤ 10<sup>5</sup></li>
/// <li>0 ≤ a,b ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let tests = scan.token::<u32>();

    for _ in 0..tests {
        let a = scan.token::<u64>();
        let b = scan.token::<u64>();

        out.write_all(if a << 1 < b || b << 1 < a || (a + b) % 3 != 0 {
            b"NO\n"
        } else {
            b"YES\n"
        })
        .unwrap();
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
3
2 1
2 2
3 3
";
        let target = b"\
YES
NO
YES
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
9
0 0
0 1
0 2
0 3
842572599 577431753
733431661 716735123
409325692 74067624
753728522 940667932
11 4
";
        let target = b"\
YES
NO
NO
NO
YES
YES
NO
YES
NO
";

        test(input, target);
    }
}
