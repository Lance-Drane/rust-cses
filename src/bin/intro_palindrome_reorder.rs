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
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut token = scan.token::<String>().into_bytes();

    let mut counter = [0_u32; 26];
    for c in &token {
        counter[(c - b'A') as usize] += 1;
    }

    let mut odd_letter = b'\0';
    for (count, letter) in counter.iter().zip(b'A'..) {
        if count & 1 == 1 {
            if odd_letter != b'\0' {
                out.write(b"NO SOLUTION\n").ok();
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

    out.write_all(&token).ok();
    out.write(&[b'\n']).ok();
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

    fn test(input: &[u8], target: &[u8]) {
        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

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
