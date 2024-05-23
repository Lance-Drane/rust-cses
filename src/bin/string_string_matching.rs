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

/// Given a string and a pattern, your task is to count the number of positions where the pattern occurs in the string.
///
/// <b>Input</b>
///
/// The first input line has a string of length n, and the second input line has a pattern of length m. Both of them consist of characters a–z.
///
/// <b>Output</b>
///
/// Print one integer: the number of occurrences.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n,m ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let string = scan.token::<String>().into_bytes();
    let pattern = scan.token::<String>().into_bytes();

    // set up longest prefix suffix array for kmp
    let mut len = 0;
    let mut longest_prefix_suffix = vec![0; pattern.len()];
    for (idx, ch) in pattern.iter().enumerate().skip(1) {
        unsafe {
            while len > 0 && pattern.get_unchecked(len) != ch {
                len = *longest_prefix_suffix.get_unchecked(len - 1);
            }
            if pattern.get_unchecked(len) == ch {
                len += 1;
            }
            *longest_prefix_suffix.get_unchecked_mut(idx) = len;
        }
    }

    // begin knuth-morris-pratt search
    len = 0;
    let matches = string
        .into_iter()
        .map(|ch| {
            unsafe {
                if len == pattern.len() {
                    len = *longest_prefix_suffix.get_unchecked(len - 1);
                }
                while len > 0 && *pattern.get_unchecked(len) != ch {
                    len = *longest_prefix_suffix.get_unchecked(len - 1);
                }
                if *pattern.get_unchecked(len) == ch {
                    len += 1;
                }
            }
            len
        })
        .filter(|size| *size == pattern.len())
        .count();
    writeln!(out, "{matches}").unwrap();
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
saippuakauppias
pp
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_identical() {
        let input = b"\
ifelloffthediscworld
ifelloffthediscworld
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_repeat() {
        let input = b"\
zzzzzzzzzz
z
";
        let target = b"\
10
";

        test(input, target);
    }

    #[test]
    fn test_repeat_threes() {
        let input = b"\
zzzzzz
zzz
";
        let target = b"\
4
";

        test(input, target);
    }

    #[test]
    fn test_no_match() {
        let input = b"\
dog
doggone
";
        let target = b"\
0
";

        test(input, target);
    }

    #[test]
    fn test_one_unique() {
        let input = b"\
AAAACAAACAAAA
AAACAAA
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_ana() {
        let input = b"\
banana
ana
";
        let target = b"\
2
";

        test(input, target);
    }
}
