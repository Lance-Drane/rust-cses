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
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
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

    #[test]
    fn test_example() {
        let input: &[u8] = b"\
5
3 2 5 1 7
";
        let target: &[u8] = b"\
5
";

        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_no_step() {
        let input: &[u8] = b"\
10
1 1 1 1 1 1 1 1 1 1
";
        let target: &[u8] = b"\
0
";

        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_one_item() {
        let input: &[u8] = b"\
1
329873232
";
        let target: &[u8] = b"\
0
";

        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_many_steps() {
        let input: &[u8] = b"\
10
1000000000 1 1 1 1 1 1 1 1 1
";
        let target: &[u8] = b"\
8999999991
";

        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }
}
