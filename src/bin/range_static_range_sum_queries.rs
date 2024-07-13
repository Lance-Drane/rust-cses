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

/// Given an array of n integers, your task is to process q queries of the form: what is the sum of values in range [a,b]?
///
/// <b>Input</b>
///
/// The first input line has two integers n and q: the number of values and queries.
///
/// The second line has n integers x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>: the array values.
///
/// Finally, there are q lines describing the queries. Each line has two integers a and b: what is the sum of values in range [a,b]?
///
/// <b>Output</b>
///
/// Print the result of each query.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n, q ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ x<sub>i</sub> ≤ 10<sup>9</sup></li>
/// <li>1 ≤ a ≤ b ≤ n</li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: usize = scan.token();
    let q: u32 = scan.token();

    let mut sum = 0;
    let values: Vec<_> = std::iter::once(0)
        .chain((0..n).map(|_| {
            sum += scan.token::<u64>();
            sum
        }))
        .collect();

    for _ in 0..q {
        let left = scan.token::<usize>() - 1;
        let right = scan.token::<usize>();
        writeln!(out, "{}", unsafe {
            values.get_unchecked(right) - values.get_unchecked(left)
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
8 4
3 2 4 5 1 1 5 3
2 4
5 6
1 8
3 3
";
        let target = b"\
11
2
24
4
";

        test(input, target);
    }
}
