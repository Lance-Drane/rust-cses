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

/// Given a string, your task is to reorder its letters in such a way that it becomes a palindrome (i.e., it reads the same forwards and backwards).
///
/// <b>Input</b>
///
/// The only input line has a string of length n consisting of characters A–Z.
///
/// <b>Output</b>
///
/// Print a palindrome consisting of the characters of the original string. You may print any valid solution. If there are no solutions, print "NO SOLUTION".
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let mut token = scan.token::<String>().into_bytes();

    let mut counter = [0_u32; 26];
    for c in &token {
        counter[(c - b'A') as usize] += 1;
    }

    let mut odd_letter = b'\0';
    for (count, letter) in counter.iter().zip(b'A'..) {
        if count & 1 == 1 {
            if odd_letter != b'\0' {
                out.write_all(b"NO SOLUTION\n").unwrap();
                return;
            }
            odd_letter = letter;
        }
    }

    let mut iter = token.iter_mut();
    for (count, letter) in counter.iter().zip(b'A'..) {
        for _ in 0..(count >> 1) {
            unsafe {
                *iter.next().unwrap_unchecked() = letter;
                *iter.next_back().unwrap_unchecked() = letter;
            }
        }
    }

    if odd_letter != b'\0' {
        unsafe {
            *iter.next().unwrap_unchecked() = odd_letter;
        }
    }

    out.write_all(&token).unwrap();
    out.write_all(&[b'\n']).unwrap();
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

    // NOTE: while CSES allows for any valid solution, we have a specific implementation.
    // Characters which come first in the alphabet come in at the beginning and end of the string
    // Characters at the end of the alphabet come in at the middle of the string

    #[test]
    fn test_example() {
        let input = b"\
AAAACACBA
";
        let target = b"\
AAACBCAAA
";

        test(input, target);
    }

    #[test]
    fn test_invalid() {
        let input = b"\
NOIX
";
        let target = b"\
NO SOLUTION
";

        test(input, target);
    }

    #[test]
    fn test_no_middle() {
        let input = b"\
REDRED
";
        let target = b"\
DERRED
";

        test(input, target);
    }

    #[test]
    fn test_long_middle() {
        let input = b"\
AADDDCC
";
        let target = b"\
ACDDDCA
";

        test(input, target);
    }
}
