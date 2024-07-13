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

/// Given an array of n integers, your task is to find for each array position the nearest position to its left having a smaller value.
///
/// <b>Input</b>
///
/// The first input line has an integer n: the size of the array.
///
/// The second line has n integers x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>: the array values.
///
/// <b>Output</b>
///
/// Print n integers: for each array position the nearest position with a smaller value. If there is no such position, print 0.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ x<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u32 = scan.token();
    let mut stack: Vec<(u32, u32)> = Vec::with_capacity(n as usize);

    'outer: for num in (1..=n).map(|idx| (scan.token::<u32>(), idx)) {
        while let Some(last) = stack.last() {
            if last.0 >= num.0 {
                stack.pop();
            } else {
                write!(out, "{} ", last.1).unwrap();
                stack.push(num);
                continue 'outer;
            }
        }
        stack.push(num);
        out.write_all(b"0 ").unwrap();
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
8
2 5 1 4 8 3 2 5
";
        let target = b"\
0 1 0 3 4 3 3 7 ";

        test(input, target);
    }
}
