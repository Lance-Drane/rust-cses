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

/// You are given an array containing n integers. Your task is to determine the longest increasing subsequence in the array, i.e., the longest subsequence where every element is larger than the previous one.
///
/// A subsequence is a sequence that can be derived from the array by deleting some elements without changing the order of the remaining elements.
///
/// <b>Input</b>
///
/// The first line contains an integer n: the size of the array.
///
/// After this there are n integers x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>: the contents of the array.
///
/// <b>Output</b>
///
/// Print the length of the longest increasing subsequence.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ k<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u32 = scan.token();
    let mut elements = Vec::with_capacity(n as usize);

    for item in (0..n).map(|_| scan.token::<u32>()) {
        match item.cmp(elements.last().unwrap_or(&0)) {
            std::cmp::Ordering::Less => {
                // find lower bound, it will always exist
                let mut low = 0;
                let mut high = elements.len();
                let mut size = high;
                while low < high {
                    let mid = low + (size >> 1);
                    if unsafe { *elements.get_unchecked(mid) } < item {
                        low = mid + 1;
                    } else {
                        high = mid;
                    }
                    size = high - low;
                }
                unsafe { *elements.get_unchecked_mut(low) = item };
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                elements.push(item);
            }
        }
    }

    writeln!(out, "{}", elements.len()).unwrap();
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
8
7 3 5 3 6 2 9 8
";
        let target = b"\
4
";

        test(input, target);
    }

    #[test]
    fn test_repeats() {
        let input = b"\
4
1 1 1 1
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_shifts() {
        let input = b"\
10
3 8 3 8 1 5 10 5 8 10
";
        let target = b"\
4
";

        test(input, target);
    }
}
