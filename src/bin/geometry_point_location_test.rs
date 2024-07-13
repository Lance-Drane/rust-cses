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

/// There is a line that goes through the points p<sub>1</sub>=(x<sub>1</sub>,y<sub>1</sub>) and p<sub>2</sub>=(x<sub>2</sub>,y<sub>2</sub>). There is also a point p<sub>3</sub>=(x<sub>3</sub>,y<sub>3</sub>).
///
/// Your task is to determine whether p<sub>3</sub> is located on the left or right side of the line or if it touches the line when we are looking from p<sub>1</sub> to p<sub>2</sub>.
///
/// <b>Input</b>
///
/// The first input line has an integer t: the number of tests.
///
/// After this, there are t lines that describe the tests. Each line has six integers: x<sub>1</sub>, y<sub>1</sub>, x<sub>2</sub>, y<sub>2</sub>, x<sub>3</sub> and y<sub>3</sub>.
///
/// <b>Output</b>
///
/// For each test, print "LEFT", "RIGHT" or "TOUCH".
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ t ≤ 10<sup>5</sup></li>
/// <li>-10<sup>9</sup> ≤ x<sub>1</sub>, y<sub>1</sub>, x<sub>2</sub>, y<sub>2</sub>, x<sub>3</sub>, y<sub>3</sub> ≤ 10<sup>9</sup></li>
/// <li>x<sub>1</sub> != x<sub>2</sub>, y<sub>1</sub> != y<sub>2</sub> </li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let t: u32 = scan.token();

    for _ in 0..t {
        let x1: i64 = scan.token();
        let y1: i64 = scan.token();
        let x2: i64 = scan.token();
        let y2: i64 = scan.token();
        let x3: i64 = scan.token();
        let y3: i64 = scan.token();

        out.write_all(
            match ((y3 - y1) * (x2 - x1) - (x3 - x1) * (y2 - y1)).cmp(&0) {
                std::cmp::Ordering::Less => b"RIGHT\n",
                std::cmp::Ordering::Equal => b"TOUCH\n",
                std::cmp::Ordering::Greater => b"LEFT\n",
            },
        )
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
3
1 1 5 3 2 3
1 1 5 3 4 1
1 1 5 3 3 2
";
        let target = b"\
LEFT
RIGHT
TOUCH
";

        test(input, target);
    }
}
