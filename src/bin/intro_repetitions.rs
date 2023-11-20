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
        // optional memory clear
        buf_str.clear();

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

/// You are given a DNA sequence: a string consisting of characters A, C, G, and T. Your task is to find the longest repetition in the sequence. This is a maximum-length substring containing only one type of character.
///
/// <b>Input</b>
///
/// The only input line contains a string of n characters.
///
/// <b>Output</b>
///
/// Print one integer: the length of the longest repetition.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let token = scan.token::<String>().into_bytes();

    writeln!(
        out,
        "{}",
        token
            .windows(2)
            .fold((1, 1), |(longest, curr_len), window| {
                if window[0] == window[1] {
                    let next_len = curr_len + 1;
                    (longest.max(next_len), next_len)
                } else {
                    (longest, 1)
                }
            })
            .0
    )
    .ok();
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin().lock());
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
    fn test_1() {
        let input = b"\
ATTCGGGA
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_one_length_string() {
        let input = b"\
A
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_one_char_repeating() {
        let input = b"\
AAAAAAAAAA
";
        let target = b"\
10
";

        test(input, target);
    }

    #[test]
    fn test_largest_at_end() {
        let input = b"\
ACCGGGTTTT
";
        let target = b"\
4
";

        test(input, target);
    }

    #[test]
    fn test_largest_at_beginning() {
        let input = b"\
AAAACCCGGT
";
        let target = b"\
4
";

        test(input, target);
    }
}
