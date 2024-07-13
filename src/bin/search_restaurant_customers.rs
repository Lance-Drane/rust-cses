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

/// You are given the arrival and leaving times of n customers in a restaurant.
///
/// What was the maximum number of customers in the restaurant at any time?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of customers.
///
/// After this, there are n lines that describe the customers. Each line has two integers a and b: the arrival and leaving times of a customer.
///
/// You may assume that all arrival and leaving times are distinct.
///
/// <b>Output</b>
///
/// Print one integer: the maximum number of customers.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a ≤ b ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let num: u32 = scan.token();
    let mut customers: Vec<(u32, i32)> = (0..num)
        .flat_map(|_| [(scan.token(), 1), (scan.token(), -1)])
        .collect();
    customers.sort_unstable_by_key(|m| m.0);

    writeln!(
        out,
        "{}",
        customers
            .into_iter()
            .fold((0, 0), |(best, current), shift| {
                let next = current + shift.1;
                (best.max(next), next)
            })
            .0
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

    // NOTE: our implementation prints out an additional space at the end of output

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
5 8
2 4
3 9
";
        let target = b"\
2
";

        test(input, target);
    }
}
