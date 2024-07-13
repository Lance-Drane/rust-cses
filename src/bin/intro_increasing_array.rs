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

/// You are given an array of n integers. You want to modify the array so that it is increasing, i.e., every element is at least as large as the previous element.
///
/// On each move, you may increase the value of any element by one. What is the minimum number of moves required?
///
/// <b>Input</b>
///
/// The first input line contains an integer n: the size of the array.
///
/// Then, the second line contains n integers x<sub>1</sub>,x<sub>2</sub>,…,x<sub>n</sub>: the contents of the array.
///
/// <b>Output</b>
///
/// Print the minimum number of moves.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ x<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let size = scan.token::<usize>();
    // we have to eagerly evaluate the first token of row 2
    let first = scan.token::<u64>();

    writeln!(
        out,
        "{}",
        (1..size)
            .map(|_| scan.token::<u64>())
            .fold((0, first), |(moves, largest), token| {
                if token < largest {
                    (largest - token + moves, largest)
                } else {
                    (moves, token)
                }
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
3 2 5 1 7
";
        let target = b"\
5
";

        test(input, target);
    }

    #[test]
    fn test_no_step() {
        let input = b"\
10
1 1 1 1 1 1 1 1 1 1
";
        let target = b"\
0
";

        test(input, target);
    }

    #[test]
    fn test_one_item() {
        let input = b"\
1
329873232
";
        let target = b"\
0
";

        test(input, target);
    }

    #[test]
    fn test_many_steps() {
        let input = b"\
10
1000000000 1 1 1 1 1 1 1 1 1
";
        let target = b"\
8999999991
";

        test(input, target);
    }
}
