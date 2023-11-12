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

/// Given an array of n integers, your task is to find the maximum sum of values in a contiguous, nonempty subarray.
///
/// <b>Input</b>
///
/// The first input line has an integer n: the size of the array.
///
/// The second line has n integers x<sub>1</sub>, x<sub>2</sub>, ..., x<sub>n</sub>: the array values.
///
/// <b>Output</b>
///
/// Print one integer: the maximum subarray sum.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>−10<sup>9</sup> ≤ x<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let size = scan.token::<u32>();
    let base = scan.token::<i64>();
    writeln!(
        out,
        "{}",
        (1..size)
            .map(|_| scan.token::<i64>())
            .fold((base, base), |(best, current), ele| {
                let next = ele.max(ele + current);
                (best.max(next), next)
            })
            .0
    )
    .ok();
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

    fn test(input: &[u8], target: &[u8]) {
        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
8
-1 3 -2 5 3 -5 2 2
";
        let target = b"\
9
";

        test(input, target);
    }

    #[test]
    fn test_one_negative() {
        let input = b"\
1
-2
";
        let target = b"\
-2
";

        test(input, target);
    }

    #[test]
    fn test_many_negative() {
        let input = b"\
5
-1 -1 -1 -1 -2
";
        let target = b"\
-1
";

        test(input, target);
    }

    #[test]
    fn test_unusual() {
        let input = b"\
10
24 7 -27 17 -67 65 -23 58 85 -39
";
        let target = b"\
185
";

        test(input, target);
    }
}
