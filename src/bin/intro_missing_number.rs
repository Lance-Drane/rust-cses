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

/// You are given all numbers between 1,2,…,n except one. Your task is to find the missing number.
///
/// <b>Input</b>
///
/// The first input line contains an integer n.
///
/// The second line contains n−1 numbers. Each number is distinct and between 1 and n (inclusive).
///
/// <b>Output</b>
///
/// Print the missing number.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>2 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let upper_bound = scan.token::<u32>();
    writeln!(
        out,
        "{}",
        (1..upper_bound).fold(upper_bound, |acc, num| acc ^ num ^ scan.token::<u32>())
    )
    .ok();
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
5
2 3 1 5
";
        let target = b"\
4
";

        test(input, target);
    }
}
