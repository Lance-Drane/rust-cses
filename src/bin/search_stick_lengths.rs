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
            std::mem::transmute::<std::str::SplitAsciiWhitespace<'_>, std::str::SplitAsciiWhitespace<'_>>(slice.split_ascii_whitespace())
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

/// There are n sticks with some lengths. Your task is to modify the sticks so that each stick has the same length.
///
/// You can either lengthen and shorten each stick. Both operations cost x where x is the difference between the new and original length.
///
/// What is the minimum total cost?
///
/// <b>Input</b>
///
/// The first input line contains an integer n: the number of sticks.
///
/// Then there are n integers: p<sub>1</sub>,p<sub>2</sub>,...,p<sub>n</sub>: the lengths of the sticks.
///
/// <b>Output</b>
///
/// Print one integer: the minimum total cost.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n,m ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ p<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: usize = scan.token();
    let mut sticks: Vec<i64> = (0..n).map(|_| scan.token()).collect();
    sticks.select_nth_unstable(n >> 1);
    let target = unsafe { *sticks.get_unchecked(n >> 1) };

    writeln!(
        out,
        "{}",
        sticks
            .into_iter()
            .fold(0, |acc, curr| acc + (curr - target).abs())
    )
    .unwrap();
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
5
2 3 1 5 2
";
        let target = b"\
5
";

        test(input, target);
    }

    #[test]
    fn test_first_powers_of_10() {
        let input = b"\
4
1 10 100 1000
";
        let target = b"\
1089
";

        test(input, target);
    }
}
