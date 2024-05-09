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

/// You are given an array that contains each number between 1 ... n exactly once. Your task is to collect the numbers from 1 to n in increasing order.
///
/// On each round, you go through the array from left to right and collect as many numbers as possible. What will be the total number of rounds?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the array size.
///
/// The next line contains n integers x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>: the numbers in the array.
///
/// <b>Output</b>
///
/// Print one integer: the number of rounds.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ k<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u32 = scan.token();
    let mut arr = vec![0; (n + 1) as usize];

    for (index, number) in (0..n).map(|idx| (idx, scan.token::<usize>())) {
        unsafe {
            *arr.get_unchecked_mut(number) = index;
        }
    }

    writeln!(
        out,
        "{}",
        arr.windows(2).filter(|w| w[1] < w[0]).count() + 1
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
4 2 1 5 3
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_asc() {
        let input = b"\
10
1 2 3 4 5 6 7 8 9 10
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_desc() {
        let input = b"\
10
10 9 8 7 6 5 4 3 2 1
";
        let target = b"\
10
";

        test(input, target);
    }
}
