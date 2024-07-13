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

/// In a movie festival n movies will be shown. You know the starting and ending time of each movie. What is the maximum number of movies you can watch entirely?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of movies.
///
/// After this, there are n lines that describe the movies. Each line has two integers a and b: the starting and ending times of a movie.
///
/// <b>Output</b>
///
/// Print one integer: the maximum number of movies.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a ≤ b ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let num: u32 = scan.token();
    let mut movies: Vec<(u32, u32)> = (0..num)
        .map(|_| {
            let start = scan.token();
            let end = scan.token();
            (end, start)
        })
        .collect();

    movies.sort_unstable_by_key(|m| m.0);

    writeln!(
        out,
        "{}",
        movies
            .into_iter()
            .fold((0, 0_u32), |(curr_end, count), (end, start)| {
                if start < curr_end {
                    (curr_end, count)
                } else {
                    (end, count + 1)
                }
            })
            .1
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
3 5
4 9
5 8
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_one() {
        let input = b"\
3
1 3
2 4
2 4
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn skip_the_first() {
        let input = b"\
3
1 1000
2 3
3 4
";
        let target = b"\
2
";

        test(input, target);
    }
}
