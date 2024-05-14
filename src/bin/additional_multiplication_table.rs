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

/// Find the middle element when the numbers in an n * n multiplication table are sorted in increasing order. It is assumed that n is odd.
///
/// For example, the 3 * 3 multiplication table is as follows:
///
/// <code>
/// 1 2 3
/// 2 4 6
/// 3 6 9
/// </code>
///
/// The numbers in increasing order are [1,2,2,3,3,4,6,6,9], so the answer is 3.
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print one integer: the answer to the task.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n < 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u64 = scan.token();

    let mut low = 1;
    let median = (n * n + 1) >> 1;
    let mut high = median; // skip an iteration, as over half the numbers are always smaller than n * n / 2

    while low < high {
        let mid = (low + high + 1) >> 1;

        // get number of values in each row that are less than the midpoint
        let mut number_smaller_in_row = n;
        let mut subsum = n;
        let mut number_smaller = 0;

        for row in 1..=n {
            while subsum >= mid {
                number_smaller_in_row -= 1;
                subsum -= row;
            }
            number_smaller += number_smaller_in_row;
            subsum += number_smaller_in_row;
        }

        if number_smaller >= median {
            high = mid - 1;
        } else {
            low = mid;
        }
    }
    writeln!(out, "{high}").unwrap();
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
3
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
5
";
        let target = b"\
8
";

        test(input, target);
    }

    #[test]
    fn test_low() {
        let input = b"\
1
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_high() {
        let input = b"\
999999
";
        let target = b"\
186682420008
";

        test(input, target);
    }
}
