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

/// You are given n cubes in a certain order, and your task is to build towers using them. Whenever two cubes are one on top of the other, the upper cube must be smaller than the lower cube.
///
/// You must process the cubes in the given order. You can always either place the cube on top of an existing tower, or begin a new tower. What is the minimum possible number of towers?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of cubes.
///
/// The next line contains n integers k<sub>1</sub>,k<sub>2</sub>,...,k<sub>n</sub>: the sizes of the cubes.
///
/// <b>Output</b>
///
/// Print one integer: the minimum number of towers.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ k<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u32 = scan.token();
    let mut towers = Vec::with_capacity(n as usize);

    for cube in (0..n).map(|_| scan.token::<u32>()) {
        if cube >= *towers.last().unwrap_or(&0) {
            towers.push(cube);
        } else {
            let mut low = 0;
            let mut high = towers.len();
            while low < high {
                let mid = (low + high) >> 1;
                if towers[mid] > cube {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }
            towers[low] = cube;
        }
    }

    writeln!(out, "{}", towers.len()).unwrap();
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
5
3 8 2 1 5
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_equal_not_on_top_of_equal() {
        let input = b"\
3
8 8 4
";
        let target = b"\
2
";

        test(input, target);
    }
}
