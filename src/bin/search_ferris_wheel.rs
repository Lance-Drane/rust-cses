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

/// There are n children who want to go to a Ferris wheel, and your task is to find a gondola for each child.
///
/// Each gondola may have one or two children in it, and in addition, the total weight in a gondola may not exceed x. You know the weight of every child.
///
/// What is the minimum number of gondolas needed for the children?
///
/// <b>Input</b>
///
/// The first input line contains two integers n and x: the number of children and the maximum allowed weight.
///
/// The next line contains n integers p<sub>1</sub>, p<sub>2</sub>, ..., p<sub>n</sub>: the weight of each child.
///
/// <b>Output</b>
///
/// Print one integer: the minimum number of gondolas.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ x ≤ 10<sup>9</sup></li>
/// <li>1 ≤ p<sub>i</sub> ≤ x</li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let num_children: u32 = scan.token();
    let max_weight: u32 = scan.token();

    let mut children: Vec<u32> = (0..num_children).map(|_| scan.token()).collect();
    children.sort_unstable();

    let mut count = 0_u32;

    let mut iter = children.into_iter();
    while let Some(thin) = iter.next() {
        loop {
            count += 1;
            if let Some(fat) = iter.next_back() {
                if thin + fat <= max_weight {
                    break;
                }
            } else {
                break;
            }
        }
    }

    writeln!(out, "{count}").unwrap();
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
4 10
7 2 3 9
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
1 1
1
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_example_3() {
        let input = b"\
5 4
2 2 2 2 2
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_example_4() {
        let input = b"\
5 3
2 2 2 2 2
";
        let target = b"\
5
";

        test(input, target);
    }
}
